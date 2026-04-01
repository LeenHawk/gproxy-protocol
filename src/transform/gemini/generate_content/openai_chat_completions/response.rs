use crate::gemini::count_tokens::types::{GeminiContentRole, GeminiFunctionCall, GeminiPart};
use crate::gemini::generate_content::response::{GeminiGenerateContentResponse, ResponseBody};
use crate::gemini::generate_content::types::{
    GeminiBlockReason, GeminiCandidate, GeminiContent, GeminiFinishReason, GeminiPromptFeedback,
    GeminiUsageMetadata,
};
use crate::gemini::types::GeminiResponseHeaders;
use crate::openai::create_chat_completions::response::OpenAiChatCompletionsResponse;
use crate::openai::create_chat_completions::types::{
    ChatCompletionFinishReason, ChatCompletionMessageToolCall,
};
use crate::transform::gemini::generate_content::utils::{
    gemini_error_response_from_openai, parse_json_object_or_empty,
};
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiChatCompletionsResponse> for GeminiGenerateContentResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiChatCompletionsResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiChatCompletionsResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let mut parts = Vec::new();
                let mut has_refusal = false;
                let mut finish_reason = Some(GeminiFinishReason::Stop);
                let choice = body.choices.into_iter().next();
                if let Some(choice) = choice {
                    finish_reason = Some(match choice.finish_reason {
                        ChatCompletionFinishReason::Stop => GeminiFinishReason::Stop,
                        ChatCompletionFinishReason::Length => GeminiFinishReason::MaxTokens,
                        ChatCompletionFinishReason::ToolCalls
                        | ChatCompletionFinishReason::FunctionCall => {
                            GeminiFinishReason::UnexpectedToolCall
                        }
                        ChatCompletionFinishReason::ContentFilter => GeminiFinishReason::Safety,
                    });

                    if let Some(text) = choice.message.content
                        && !text.is_empty()
                    {
                        parts.push(GeminiPart {
                            text: Some(text),
                            ..GeminiPart::default()
                        });
                    }

                    if let Some(refusal) = choice.message.refusal {
                        has_refusal = true;
                        if !refusal.is_empty() {
                            parts.push(GeminiPart {
                                text: Some(refusal),
                                ..GeminiPart::default()
                            });
                        }
                    }

                    if let Some(function_call) = choice.message.function_call {
                        parts.push(GeminiPart {
                            function_call: Some(GeminiFunctionCall {
                                id: Some("function_call".to_string()),
                                name: function_call.name,
                                args: Some(parse_json_object_or_empty(&function_call.arguments)),
                            }),
                            ..GeminiPart::default()
                        });
                    }

                    if let Some(tool_calls) = choice.message.tool_calls {
                        for call in tool_calls {
                            match call {
                                ChatCompletionMessageToolCall::Function(call) => {
                                    parts.push(GeminiPart {
                                        function_call: Some(GeminiFunctionCall {
                                            id: Some(call.id),
                                            name: call.function.name,
                                            args: Some(parse_json_object_or_empty(
                                                &call.function.arguments,
                                            )),
                                        }),
                                        ..GeminiPart::default()
                                    });
                                }
                                ChatCompletionMessageToolCall::Custom(call) => {
                                    parts.push(GeminiPart {
                                        function_call: Some(GeminiFunctionCall {
                                            id: Some(call.id),
                                            name: call.custom.name,
                                            args: Some(parse_json_object_or_empty(
                                                &call.custom.input,
                                            )),
                                        }),
                                        ..GeminiPart::default()
                                    });
                                }
                            }
                        }
                    }
                }

                if parts.is_empty() {
                    parts.push(GeminiPart {
                        text: Some(String::new()),
                        ..GeminiPart::default()
                    });
                }

                let prompt_feedback =
                    if has_refusal || matches!(finish_reason, Some(GeminiFinishReason::Safety)) {
                        Some(GeminiPromptFeedback {
                            block_reason: Some(GeminiBlockReason::Safety),
                            safety_ratings: None,
                        })
                    } else {
                        None
                    };

                let usage_metadata = body.usage.map(|usage| GeminiUsageMetadata {
                    prompt_token_count: Some(usage.prompt_tokens),
                    cached_content_token_count: usage
                        .prompt_tokens_details
                        .as_ref()
                        .and_then(|details| details.cached_tokens),
                    candidates_token_count: Some(usage.completion_tokens),
                    thoughts_token_count: usage
                        .completion_tokens_details
                        .as_ref()
                        .and_then(|details| details.reasoning_tokens),
                    total_token_count: Some(usage.total_tokens),
                    ..GeminiUsageMetadata::default()
                });

                GeminiGenerateContentResponse::Success {
                    stats_code,
                    headers: GeminiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ResponseBody {
                        candidates: Some(vec![GeminiCandidate {
                            content: Some(GeminiContent {
                                parts,
                                role: Some(GeminiContentRole::Model),
                            }),
                            finish_reason,
                            index: Some(0),
                            ..GeminiCandidate::default()
                        }]),
                        prompt_feedback,
                        usage_metadata,
                        model_version: Some(body.model),
                        response_id: Some(body.id),
                        model_status: None,
                    },
                }
            }
            OpenAiChatCompletionsResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiGenerateContentResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_error_response_from_openai(stats_code, body),
            },
        })
    }
}
