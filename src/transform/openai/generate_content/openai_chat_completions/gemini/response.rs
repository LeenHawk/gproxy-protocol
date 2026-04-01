use super::utils::{
    gemini_citation_annotations, gemini_function_response_to_text, gemini_logprobs,
    json_object_to_string, prompt_feedback_refusal_text,
};
use crate::gemini::generate_content::response::GeminiGenerateContentResponse;
use crate::gemini::generate_content::types as gt;
use crate::openai::create_chat_completions::response::OpenAiChatCompletionsResponse;
use crate::openai::create_chat_completions::types as ct;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::openai::model_list::gemini::utils::{
    openai_error_response_from_gemini, strip_models_prefix,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiGenerateContentResponse> for OpenAiChatCompletionsResponse {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiGenerateContentResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let prompt_feedback = body.prompt_feedback;
                let mut choices = Vec::new();

                for (candidate_pos, candidate) in
                    body.candidates.unwrap_or_default().into_iter().enumerate()
                {
                    let index = candidate.index.unwrap_or(candidate_pos as u32);
                    let candidate_finish_message = candidate.finish_message.clone();
                    let mut content_parts = Vec::new();
                    let mut refusal_parts = Vec::new();
                    let mut function_tool_calls = Vec::new();
                    let mut custom_tool_calls = Vec::new();

                    if let Some(content) = candidate.content {
                        for (part_index, part) in content.parts.into_iter().enumerate() {
                            if part.thought.unwrap_or(false) {
                                continue;
                            }

                            if let Some(function_call) = part.function_call {
                                let call_id = function_call.id.unwrap_or_else(|| {
                                    format!("candidate_{index}_tool_{part_index}")
                                });
                                function_tool_calls.push(
                                    ct::ChatCompletionMessageFunctionToolCall {
                                        id: call_id,
                                        function: ct::ChatCompletionFunctionCall {
                                            arguments: function_call
                                                .args
                                                .as_ref()
                                                .map(json_object_to_string)
                                                .unwrap_or_else(|| "{}".to_string()),
                                            name: function_call.name,
                                        },
                                        type_:
                                            ct::ChatCompletionMessageFunctionToolCallType::Function,
                                    },
                                );
                            }

                            if let Some(executable_code) = part.executable_code {
                                custom_tool_calls.push(ct::ChatCompletionMessageCustomToolCall {
                                    id: format!("code_execution_{index}_{part_index}"),
                                    custom: ct::ChatCompletionMessageCustomToolCallPayload {
                                        input: executable_code.code,
                                        name: "code_execution".to_string(),
                                    },
                                    type_: ct::ChatCompletionMessageCustomToolCallType::Custom,
                                });
                            }

                            if let Some(function_response) = part.function_response {
                                let output_text =
                                    gemini_function_response_to_text(function_response);
                                if !output_text.is_empty() {
                                    content_parts.push(output_text);
                                }
                            }

                            if let Some(code_execution_result) = part.code_execution_result
                                && let Some(output_text) = code_execution_result.output
                                && !output_text.is_empty()
                            {
                                content_parts.push(output_text);
                            }

                            if let Some(text) = part.text
                                && !text.is_empty()
                            {
                                content_parts.push(text);
                            }

                            if let Some(inline_data) = part.inline_data {
                                content_parts.push(format!(
                                    "data:{};base64,{}",
                                    inline_data.mime_type, inline_data.data
                                ));
                            }

                            if let Some(file_data) = part.file_data
                                && !file_data.file_uri.is_empty()
                            {
                                content_parts.push(file_data.file_uri);
                            }
                        }
                    }

                    if content_parts.is_empty()
                        && let Some(finish_message) = candidate_finish_message.clone()
                        && !finish_message.is_empty()
                    {
                        content_parts.push(finish_message);
                    }

                    let mut tool_calls = function_tool_calls
                        .into_iter()
                        .map(ct::ChatCompletionMessageToolCall::Function)
                        .collect::<Vec<_>>();
                    tool_calls.extend(
                        custom_tool_calls
                            .into_iter()
                            .map(ct::ChatCompletionMessageToolCall::Custom),
                    );

                    let finish_reason = match candidate.finish_reason.as_ref() {
                        Some(gt::GeminiFinishReason::MaxTokens) => {
                            ct::ChatCompletionFinishReason::Length
                        }
                        Some(
                            gt::GeminiFinishReason::Safety
                            | gt::GeminiFinishReason::Recitation
                            | gt::GeminiFinishReason::Blocklist
                            | gt::GeminiFinishReason::ProhibitedContent
                            | gt::GeminiFinishReason::Spii
                            | gt::GeminiFinishReason::ImageSafety
                            | gt::GeminiFinishReason::ImageProhibitedContent
                            | gt::GeminiFinishReason::ImageRecitation,
                        ) => ct::ChatCompletionFinishReason::ContentFilter,
                        _ if !tool_calls.is_empty() => ct::ChatCompletionFinishReason::ToolCalls,
                        _ => ct::ChatCompletionFinishReason::Stop,
                    };

                    if matches!(finish_reason, ct::ChatCompletionFinishReason::ContentFilter)
                        && refusal_parts.is_empty()
                        && let Some(finish_message) = candidate_finish_message
                        && !finish_message.is_empty()
                    {
                        refusal_parts.push(finish_message);
                    }

                    let function_call = tool_calls.iter().find_map(|call| match call {
                        ct::ChatCompletionMessageToolCall::Function(call) => {
                            Some(call.function.clone())
                        }
                        ct::ChatCompletionMessageToolCall::Custom(_) => None,
                    });

                    let annotations =
                        gemini_citation_annotations(candidate.citation_metadata.as_ref());

                    choices.push(ct::ChatCompletionChoice {
                        finish_reason,
                        index,
                        logprobs: gemini_logprobs(candidate.logprobs_result.as_ref()),
                        message: ct::ChatCompletionMessage {
                            content: if content_parts.is_empty() {
                                None
                            } else {
                                Some(content_parts.join("\n"))
                            },
                            reasoning_content: None,
                            reasoning_details: None,
                            refusal: if refusal_parts.is_empty() {
                                None
                            } else {
                                Some(refusal_parts.join("\n"))
                            },
                            role: ct::ChatCompletionAssistantRole::Assistant,
                            annotations: if annotations.is_empty() {
                                None
                            } else {
                                Some(annotations)
                            },
                            audio: None,
                            function_call,
                            tool_calls: if tool_calls.is_empty() {
                                None
                            } else {
                                Some(tool_calls)
                            },
                        },
                    });
                }

                if choices.is_empty() {
                    let refusal = prompt_feedback_refusal_text(prompt_feedback.as_ref());
                    let finish_reason = if refusal.is_some() {
                        ct::ChatCompletionFinishReason::ContentFilter
                    } else {
                        ct::ChatCompletionFinishReason::Stop
                    };

                    choices.push(ct::ChatCompletionChoice {
                        finish_reason,
                        index: 0,
                        logprobs: None,
                        message: ct::ChatCompletionMessage {
                            content: None,
                            reasoning_content: None,
                            reasoning_details: None,
                            refusal,
                            role: ct::ChatCompletionAssistantRole::Assistant,
                            annotations: None,
                            audio: None,
                            function_call: None,
                            tool_calls: None,
                        },
                    });
                }

                let id = body
                    .response_id
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| "gemini-response".to_string());
                let model = body
                    .model_version
                    .as_deref()
                    .map(strip_models_prefix)
                    .unwrap_or_default();

                OpenAiChatCompletionsResponse::Success {
                    stats_code,
                    headers: OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ct::ChatCompletion {
                        id,
                        choices,
                        created: 0,
                        model,
                        object: ct::ChatCompletionObject::ChatCompletion,
                        service_tier: None,
                        system_fingerprint: None,
                        usage: body.usage_metadata.map(|usage| {
                            let prompt_tokens = usage
                                .prompt_token_count
                                .unwrap_or(0)
                                .saturating_add(usage.tool_use_prompt_token_count.unwrap_or(0));
                            let completion_tokens = usage
                                .candidates_token_count
                                .unwrap_or(0)
                                .saturating_add(usage.thoughts_token_count.unwrap_or(0));
                            let total_tokens = usage
                                .total_token_count
                                .unwrap_or_else(|| prompt_tokens.saturating_add(completion_tokens));

                            ct::CompletionUsage {
                                completion_tokens,
                                prompt_tokens,
                                total_tokens,
                                completion_tokens_details: Some(ct::CompletionTokensDetails {
                                    accepted_prediction_tokens: None,
                                    audio_tokens: None,
                                    reasoning_tokens: Some(usage.thoughts_token_count.unwrap_or(0)),
                                    rejected_prediction_tokens: None,
                                }),
                                prompt_tokens_details: Some(ct::PromptTokensDetails {
                                    audio_tokens: None,
                                    cached_tokens: Some(
                                        usage.cached_content_token_count.unwrap_or(0),
                                    ),
                                }),
                            }
                        }),
                    },
                }
            }
            GeminiGenerateContentResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiChatCompletionsResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
