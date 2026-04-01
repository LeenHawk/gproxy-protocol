use crate::claude::count_tokens::types::BetaToolUseBlockType;
use crate::claude::create_message::response::ClaudeCreateMessageResponse;
use crate::claude::create_message::types::{
    BetaContentBlock, BetaMessage, BetaMessageRole, BetaMessageType, BetaServiceTier,
    BetaStopReason, BetaTextBlock, BetaTextBlockType, Model,
};
use crate::claude::types::ClaudeResponseHeaders;
use crate::openai::create_chat_completions::response::OpenAiChatCompletionsResponse;
use crate::openai::create_chat_completions::types::{
    ChatCompletionFinishReason, ChatCompletionMessageToolCall, ChatCompletionServiceTier,
};
use crate::transform::claude::generate_content::utils::{
    beta_usage_from_counts, parse_json_object_or_empty,
};
use crate::transform::claude::utils::beta_error_response_from_status_message;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiChatCompletionsResponse> for ClaudeCreateMessageResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiChatCompletionsResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiChatCompletionsResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let choice = body.choices.into_iter().next();
                let mut content = Vec::new();
                let mut has_tool_use = false;
                let mut has_refusal = false;
                let finish_reason = choice.as_ref().map(|choice| choice.finish_reason.clone());

                if let Some(choice) = choice {
                    if let Some(text) = choice.message.content {
                        content.push(BetaContentBlock::Text(BetaTextBlock {
                            citations: None,
                            text,
                            type_: BetaTextBlockType::Text,
                        }));
                    }
                    if let Some(refusal) = choice.message.refusal {
                        has_refusal = true;
                        content.push(BetaContentBlock::Text(BetaTextBlock {
                            citations: None,
                            text: refusal,
                            type_: BetaTextBlockType::Text,
                        }));
                    }
                    if let Some(function_call) = choice.message.function_call {
                        has_tool_use = true;
                        content.push(BetaContentBlock::ToolUse(
                            crate::claude::create_message::types::BetaToolUseBlock {
                                id: "function_call".to_string(),
                                input: parse_json_object_or_empty(&function_call.arguments),
                                name: function_call.name,
                                type_: BetaToolUseBlockType::ToolUse,
                                cache_control: None,
                                caller: None,
                            },
                        ));
                    }
                    if let Some(tool_calls) = choice.message.tool_calls {
                        for call in tool_calls {
                            match call {
                                ChatCompletionMessageToolCall::Function(call) => {
                                    has_tool_use = true;
                                    content.push(BetaContentBlock::ToolUse(
                                        crate::claude::create_message::types::BetaToolUseBlock {
                                            id: call.id,
                                            input: parse_json_object_or_empty(
                                                &call.function.arguments,
                                            ),
                                            name: call.function.name,
                                            type_: BetaToolUseBlockType::ToolUse,
                                            cache_control: None,
                                            caller: None,
                                        },
                                    ));
                                }
                                ChatCompletionMessageToolCall::Custom(call) => {
                                    has_tool_use = true;
                                    content.push(BetaContentBlock::ToolUse(
                                        crate::claude::create_message::types::BetaToolUseBlock {
                                            id: call.id,
                                            input: parse_json_object_or_empty(&call.custom.input),
                                            name: call.custom.name,
                                            type_: BetaToolUseBlockType::ToolUse,
                                            cache_control: None,
                                            caller: None,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }

                if content.is_empty() {
                    content.push(BetaContentBlock::Text(BetaTextBlock {
                        citations: None,
                        text: String::new(),
                        type_: BetaTextBlockType::Text,
                    }));
                }

                let stop_reason = match finish_reason {
                    Some(ChatCompletionFinishReason::Stop) => Some(BetaStopReason::EndTurn),
                    Some(ChatCompletionFinishReason::Length) => Some(BetaStopReason::MaxTokens),
                    Some(ChatCompletionFinishReason::ToolCalls)
                    | Some(ChatCompletionFinishReason::FunctionCall) => {
                        Some(BetaStopReason::ToolUse)
                    }
                    Some(ChatCompletionFinishReason::ContentFilter) => {
                        Some(BetaStopReason::Refusal)
                    }
                    _ => {
                        if has_tool_use {
                            Some(BetaStopReason::ToolUse)
                        } else if has_refusal {
                            Some(BetaStopReason::Refusal)
                        } else {
                            Some(BetaStopReason::EndTurn)
                        }
                    }
                };

                let (input_tokens, cached_tokens, output_tokens) = body
                    .usage
                    .as_ref()
                    .map(|usage| {
                        let cached_tokens = usage
                            .prompt_tokens_details
                            .as_ref()
                            .and_then(|details| details.cached_tokens)
                            .unwrap_or(0);
                        let total_input_tokens = if usage.total_tokens >= usage.completion_tokens {
                            usage.total_tokens.saturating_sub(usage.completion_tokens)
                        } else {
                            usage.prompt_tokens
                        };
                        (
                            total_input_tokens.saturating_sub(cached_tokens),
                            cached_tokens,
                            usage.completion_tokens,
                        )
                    })
                    .unwrap_or((0, 0, 0));
                let service_tier = match body.service_tier {
                    Some(ChatCompletionServiceTier::Priority) => BetaServiceTier::Priority,
                    _ => BetaServiceTier::Standard,
                };
                let usage = beta_usage_from_counts(
                    input_tokens,
                    cached_tokens,
                    output_tokens,
                    service_tier,
                );

                ClaudeCreateMessageResponse::Success {
                    stats_code,
                    headers: ClaudeResponseHeaders {
                        extra: headers.extra,
                    },
                    body: BetaMessage {
                        id: body.id,
                        container: None,
                        content,
                        context_management: None,
                        model: Model::Custom(body.model),
                        role: BetaMessageRole::Assistant,
                        stop_reason,
                        stop_sequence: None,
                        type_: BetaMessageType::Message,
                        usage,
                    },
                }
            }
            OpenAiChatCompletionsResponse::Error {
                stats_code,
                headers,
                body,
            } => ClaudeCreateMessageResponse::Error {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_error_response_from_status_message(stats_code, body.error.message),
            },
        })
    }
}
