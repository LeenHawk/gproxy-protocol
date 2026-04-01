use crate::claude::count_tokens::types::{BetaThinkingBlockType, BetaToolUseBlockType};
use crate::claude::create_message::response::ClaudeCreateMessageResponse;
use crate::claude::create_message::types::{
    BetaContentBlock, BetaMessage, BetaMessageRole, BetaMessageType, BetaServiceTier,
    BetaStopReason, BetaTextBlock, BetaTextBlockType, Model,
};
use crate::claude::types::ClaudeResponseHeaders;
use crate::gemini::generate_content::response::GeminiGenerateContentResponse;
use crate::gemini::generate_content::types::{GeminiBlockReason, GeminiFinishReason};
use crate::transform::claude::generate_content::utils::beta_usage_from_counts;
use crate::transform::claude::utils::beta_error_response_from_status_message;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiGenerateContentResponse> for ClaudeCreateMessageResponse {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiGenerateContentResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let mut content = Vec::new();
                let mut has_tool_use = false;

                let candidate = body
                    .candidates
                    .clone()
                    .and_then(|items| items.into_iter().next());
                if let Some(candidate) = candidate {
                    if let Some(candidate_content) = candidate.content {
                        for (idx, part) in candidate_content.parts.into_iter().enumerate() {
                            if part.thought.unwrap_or(false) {
                                if let Some(text) = part.text {
                                    content.push(BetaContentBlock::Thinking(
                                        crate::claude::create_message::types::BetaThinkingBlock {
                                            signature: part
                                                .thought_signature
                                                .unwrap_or_else(|| format!("thought_{idx}")),
                                            thinking: text,
                                            type_: BetaThinkingBlockType::Thinking,
                                        },
                                    ));
                                }
                            } else if let Some(text) = part.text {
                                content.push(BetaContentBlock::Text(BetaTextBlock {
                                    citations: None,
                                    text,
                                    type_: BetaTextBlockType::Text,
                                }));
                            }

                            if let Some(function_call) = part.function_call {
                                has_tool_use = true;
                                content.push(BetaContentBlock::ToolUse(
                                    crate::claude::create_message::types::BetaToolUseBlock {
                                        id: function_call
                                            .id
                                            .unwrap_or_else(|| format!("tool_call_{idx}")),
                                        input: function_call.args.unwrap_or_default(),
                                        name: function_call.name,
                                        type_: BetaToolUseBlockType::ToolUse,
                                        cache_control: None,
                                        caller: None,
                                    },
                                ));
                            }

                            if let Some(function_response) = part.function_response {
                                let response_text =
                                    serde_json::to_string(&function_response.response)
                                        .unwrap_or_default();
                                if !response_text.is_empty() {
                                    content.push(BetaContentBlock::Text(BetaTextBlock {
                                        citations: None,
                                        text: response_text,
                                        type_: BetaTextBlockType::Text,
                                    }));
                                }
                            }

                            if let Some(executable_code) = part.executable_code {
                                content.push(BetaContentBlock::Text(BetaTextBlock {
                                    citations: None,
                                    text: executable_code.code,
                                    type_: BetaTextBlockType::Text,
                                }));
                            }

                            if let Some(code_execution_result) = part.code_execution_result
                                && let Some(output) = code_execution_result.output
                                && !output.is_empty()
                            {
                                content.push(BetaContentBlock::Text(BetaTextBlock {
                                    citations: None,
                                    text: output,
                                    type_: BetaTextBlockType::Text,
                                }));
                            }

                            if let Some(file_data) = part.file_data {
                                content.push(BetaContentBlock::Text(BetaTextBlock {
                                    citations: None,
                                    text: file_data.file_uri,
                                    type_: BetaTextBlockType::Text,
                                }));
                            }
                        }
                    }

                    if content.is_empty() {
                        content.push(BetaContentBlock::Text(BetaTextBlock {
                            citations: None,
                            text: candidate.finish_message.unwrap_or_default(),
                            type_: BetaTextBlockType::Text,
                        }));
                    }

                    let stop_reason = match candidate.finish_reason {
                        Some(GeminiFinishReason::MaxTokens) => Some(BetaStopReason::MaxTokens),
                        Some(GeminiFinishReason::MalformedFunctionCall)
                        | Some(GeminiFinishReason::UnexpectedToolCall)
                        | Some(GeminiFinishReason::TooManyToolCalls)
                        | Some(GeminiFinishReason::MissingThoughtSignature) => {
                            Some(BetaStopReason::ToolUse)
                        }
                        Some(GeminiFinishReason::Safety)
                        | Some(GeminiFinishReason::Recitation)
                        | Some(GeminiFinishReason::Blocklist)
                        | Some(GeminiFinishReason::ProhibitedContent)
                        | Some(GeminiFinishReason::Spii)
                        | Some(GeminiFinishReason::ImageSafety)
                        | Some(GeminiFinishReason::ImageProhibitedContent)
                        | Some(GeminiFinishReason::ImageRecitation) => {
                            Some(BetaStopReason::Refusal)
                        }
                        _ => {
                            if has_tool_use {
                                Some(BetaStopReason::ToolUse)
                            } else {
                                Some(BetaStopReason::EndTurn)
                            }
                        }
                    };

                    let usage_metadata = body.usage_metadata.unwrap_or_default();
                    let prompt_input_tokens = usage_metadata
                        .prompt_token_count
                        .unwrap_or(0)
                        .saturating_add(usage_metadata.tool_use_prompt_token_count.unwrap_or(0));
                    let cached_tokens = usage_metadata.cached_content_token_count.unwrap_or(0);
                    let output_tokens = usage_metadata
                        .candidates_token_count
                        .unwrap_or(0)
                        .saturating_add(usage_metadata.thoughts_token_count.unwrap_or(0));
                    let total_input_tokens = usage_metadata
                        .total_token_count
                        .map(|total| total.saturating_sub(output_tokens))
                        .unwrap_or_else(|| prompt_input_tokens.saturating_add(cached_tokens));
                    let input_tokens = total_input_tokens.saturating_sub(cached_tokens);
                    let usage = beta_usage_from_counts(
                        input_tokens,
                        cached_tokens,
                        output_tokens,
                        BetaServiceTier::Standard,
                    );

                    ClaudeCreateMessageResponse::Success {
                        stats_code,
                        headers: ClaudeResponseHeaders {
                            extra: headers.extra,
                        },
                        body: BetaMessage {
                            id: body.response_id.unwrap_or_default(),
                            container: None,
                            content,
                            context_management: None,
                            model: Model::Custom(body.model_version.unwrap_or_default()),
                            role: BetaMessageRole::Assistant,
                            stop_reason,
                            stop_sequence: None,
                            type_: BetaMessageType::Message,
                            usage,
                        },
                    }
                } else {
                    let block_reason = body
                        .prompt_feedback
                        .as_ref()
                        .and_then(|feedback| feedback.block_reason.as_ref());
                    let stop_reason = match block_reason {
                        Some(GeminiBlockReason::Safety)
                        | Some(GeminiBlockReason::Blocklist)
                        | Some(GeminiBlockReason::ProhibitedContent)
                        | Some(GeminiBlockReason::ImageSafety) => Some(BetaStopReason::Refusal),
                        _ => Some(BetaStopReason::EndTurn),
                    };
                    let fallback_text = match block_reason {
                        Some(GeminiBlockReason::Safety) => "blocked_by_safety".to_string(),
                        Some(GeminiBlockReason::Other) => "blocked".to_string(),
                        Some(GeminiBlockReason::Blocklist) => "blocked_by_blocklist".to_string(),
                        Some(GeminiBlockReason::ProhibitedContent) => {
                            "blocked_by_prohibited_content".to_string()
                        }
                        Some(GeminiBlockReason::ImageSafety) => {
                            "blocked_by_image_safety".to_string()
                        }
                        Some(GeminiBlockReason::BlockReasonUnspecified) | None => String::new(),
                    };
                    let usage = beta_usage_from_counts(0, 0, 0, BetaServiceTier::Standard);
                    ClaudeCreateMessageResponse::Success {
                        stats_code,
                        headers: ClaudeResponseHeaders {
                            extra: headers.extra,
                        },
                        body: BetaMessage {
                            id: body.response_id.unwrap_or_default(),
                            container: None,
                            content: vec![BetaContentBlock::Text(BetaTextBlock {
                                citations: None,
                                text: fallback_text,
                                type_: BetaTextBlockType::Text,
                            })],
                            context_management: None,
                            model: Model::Custom(body.model_version.unwrap_or_default()),
                            role: BetaMessageRole::Assistant,
                            stop_reason,
                            stop_sequence: None,
                            type_: BetaMessageType::Message,
                            usage,
                        },
                    }
                }
            }
            GeminiGenerateContentResponse::Error {
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
