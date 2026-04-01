use crate::gemini::generate_content::response::GeminiGenerateContentResponse;
use crate::openai::compact_response::response::OpenAiCompactResponse;
use crate::openai::compact_response::response::{
    OpenAiCompactedResponseObject, ResponseBody as CompactResponseBody,
};
use crate::openai::compact_response::types as cpt;
use crate::openai::count_tokens::types as ot;
use crate::transform::openai::generate_content::openai_chat_completions::gemini::utils::{
    gemini_function_response_to_text, json_object_to_string, prompt_feedback_refusal_text,
};
use crate::transform::openai::generate_content::openai_response::gemini::utils::{
    gemini_citation_annotations, gemini_logprobs,
};
use crate::transform::openai::model_list::gemini::utils::openai_error_response_from_gemini;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiGenerateContentResponse> for OpenAiCompactResponse {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiGenerateContentResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let response_id = body.response_id.unwrap_or_else(|| "compact".to_string());
                let prompt_feedback = body.prompt_feedback;
                let mut output = Vec::new();

                for (candidate_index, candidate) in
                    body.candidates.unwrap_or_default().into_iter().enumerate()
                {
                    let mut message_content = Vec::new();

                    if let Some(content) = candidate.content.clone() {
                        for part in content.parts {
                            if let Some(text) = part.text
                                && !text.is_empty()
                            {
                                message_content.push(
                                    cpt::CompactedResponseMessageContent::OutputText(
                                        ot::ResponseOutputText {
                                            annotations: gemini_citation_annotations(
                                                candidate.citation_metadata.as_ref(),
                                            ),
                                            logprobs: gemini_logprobs(
                                                candidate.logprobs_result.as_ref(),
                                            ),
                                            text,
                                            type_: ot::ResponseOutputTextType::OutputText,
                                        },
                                    ),
                                );
                            }

                            if let Some(function_call) = part.function_call {
                                output.push(cpt::CompactedResponseOutputItem::FunctionToolCall(
                                    ot::ResponseFunctionToolCall {
                                        arguments: function_call
                                            .args
                                            .as_ref()
                                            .map(json_object_to_string)
                                            .unwrap_or_else(|| "{}".to_string()),
                                        call_id: function_call.id.unwrap_or_else(|| {
                                            format!(
                                                "{}_candidate_{}_tool_call",
                                                response_id, candidate_index
                                            )
                                        }),
                                        name: function_call.name,
                                        type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                        id: None,
                                        status: Some(ot::ResponseItemStatus::Completed),
                                    },
                                ));
                            }

                            if let Some(function_response) = part.function_response {
                                output.push(cpt::CompactedResponseOutputItem::FunctionCallOutput(
                                    ot::ResponseFunctionCallOutput {
                                        call_id: function_response
                                            .id
                                            .clone()
                                            .unwrap_or_else(|| function_response.name.clone()),
                                        output: ot::ResponseFunctionCallOutputContent::Text(
                                            gemini_function_response_to_text(function_response),
                                        ),
                                        type_:
                                            ot::ResponseFunctionCallOutputType::FunctionCallOutput,
                                        id: None,
                                        status: Some(ot::ResponseItemStatus::Completed),
                                    },
                                ));
                            }

                            if let Some(inline_data) = part.inline_data {
                                message_content.push(
                                    cpt::CompactedResponseMessageContent::OutputText(
                                        ot::ResponseOutputText {
                                            annotations: Vec::new(),
                                            logprobs: None,
                                            text: format!(
                                                "data:{};base64,{}",
                                                inline_data.mime_type, inline_data.data
                                            ),
                                            type_: ot::ResponseOutputTextType::OutputText,
                                        },
                                    ),
                                );
                            }

                            if let Some(file_data) = part.file_data {
                                message_content.push(
                                    cpt::CompactedResponseMessageContent::OutputText(
                                        ot::ResponseOutputText {
                                            annotations: Vec::new(),
                                            logprobs: None,
                                            text: file_data.file_uri,
                                            type_: ot::ResponseOutputTextType::OutputText,
                                        },
                                    ),
                                );
                            }

                            if let Some(executable_code) = part.executable_code {
                                message_content.push(
                                    cpt::CompactedResponseMessageContent::OutputText(
                                        ot::ResponseOutputText {
                                            annotations: Vec::new(),
                                            logprobs: None,
                                            text: executable_code.code,
                                            type_: ot::ResponseOutputTextType::OutputText,
                                        },
                                    ),
                                );
                            }

                            if let Some(code_execution_result) = part.code_execution_result
                                && let Some(output_text) = code_execution_result.output
                                && !output_text.is_empty()
                            {
                                message_content.push(
                                    cpt::CompactedResponseMessageContent::OutputText(
                                        ot::ResponseOutputText {
                                            annotations: Vec::new(),
                                            logprobs: None,
                                            text: output_text,
                                            type_: ot::ResponseOutputTextType::OutputText,
                                        },
                                    ),
                                );
                            }
                        }
                    }

                    if message_content.is_empty()
                        && let Some(finish_message) = candidate.finish_message
                        && !finish_message.is_empty()
                    {
                        message_content.push(cpt::CompactedResponseMessageContent::OutputText(
                            ot::ResponseOutputText {
                                annotations: Vec::new(),
                                logprobs: None,
                                text: finish_message,
                                type_: ot::ResponseOutputTextType::OutputText,
                            },
                        ));
                    }

                    if !message_content.is_empty() {
                        let status = match candidate.finish_reason.as_ref() {
                            Some(
                                crate::gemini::generate_content::types::GeminiFinishReason::MaxTokens
                                | crate::gemini::generate_content::types::GeminiFinishReason::Safety
                                | crate::gemini::generate_content::types::GeminiFinishReason::Recitation
                                | crate::gemini::generate_content::types::GeminiFinishReason::Blocklist
                                | crate::gemini::generate_content::types::GeminiFinishReason::ProhibitedContent
                                | crate::gemini::generate_content::types::GeminiFinishReason::Spii
                                | crate::gemini::generate_content::types::GeminiFinishReason::ImageSafety
                                | crate::gemini::generate_content::types::GeminiFinishReason::ImageProhibitedContent
                                | crate::gemini::generate_content::types::GeminiFinishReason::ImageRecitation,
                            ) => ot::ResponseItemStatus::Incomplete,
                            _ => ot::ResponseItemStatus::Completed,
                        };
                        output.push(cpt::CompactedResponseOutputItem::Message(
                            cpt::CompactedResponseMessage {
                                id: format!("{}_message_{}", response_id, candidate_index),
                                content: message_content,
                                role: cpt::CompactedResponseMessageRole::Assistant,
                                status,
                                type_: cpt::CompactedResponseMessageType::Message,
                            },
                        ));
                    }
                }

                if output.is_empty()
                    && let Some(refusal) = prompt_feedback_refusal_text(prompt_feedback.as_ref())
                {
                    output.push(cpt::CompactedResponseOutputItem::Message(
                        cpt::CompactedResponseMessage {
                            id: format!("{}_message_0", response_id),
                            content: vec![cpt::CompactedResponseMessageContent::Refusal(
                                ot::ResponseOutputRefusal {
                                    refusal,
                                    type_: ot::ResponseOutputRefusalType::Refusal,
                                },
                            )],
                            role: cpt::CompactedResponseMessageRole::Assistant,
                            status: ot::ResponseItemStatus::Incomplete,
                            type_: cpt::CompactedResponseMessageType::Message,
                        },
                    ));
                }

                let usage = body
                    .usage_metadata
                    .map(|usage| {
                        let input_tokens = usage
                            .prompt_token_count
                            .unwrap_or(0)
                            .saturating_add(usage.tool_use_prompt_token_count.unwrap_or(0));
                        let output_tokens = usage
                            .candidates_token_count
                            .unwrap_or(0)
                            .saturating_add(usage.thoughts_token_count.unwrap_or(0));
                        cpt::ResponseUsage {
                            input_tokens,
                            input_tokens_details: cpt::ResponseInputTokensDetails {
                                cached_tokens: usage.cached_content_token_count.unwrap_or(0),
                            },
                            output_tokens,
                            output_tokens_details: cpt::ResponseOutputTokensDetails {
                                reasoning_tokens: usage.thoughts_token_count.unwrap_or(0),
                            },
                            total_tokens: usage
                                .total_token_count
                                .unwrap_or_else(|| input_tokens.saturating_add(output_tokens)),
                        }
                    })
                    .unwrap_or(cpt::ResponseUsage {
                        input_tokens: 0,
                        input_tokens_details: cpt::ResponseInputTokensDetails { cached_tokens: 0 },
                        output_tokens: 0,
                        output_tokens_details: cpt::ResponseOutputTokensDetails {
                            reasoning_tokens: 0,
                        },
                        total_tokens: 0,
                    });

                OpenAiCompactResponse::Success {
                    stats_code,
                    headers: crate::openai::types::OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: CompactResponseBody {
                        id: response_id,
                        created_at: 0,
                        object: OpenAiCompactedResponseObject::ResponseCompaction,
                        output,
                        usage,
                    },
                }
            }
            GeminiGenerateContentResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCompactResponse::Error {
                stats_code,
                headers: crate::openai::types::OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
