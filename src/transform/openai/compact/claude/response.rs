use crate::claude::count_tokens::types::BetaServerToolUseName;
use crate::claude::create_message::response::ClaudeCreateMessageResponse;
use crate::claude::create_message::types as ct;
use crate::openai::compact_response::response::OpenAiCompactResponse;
use crate::openai::compact_response::response::{
    OpenAiCompactedResponseObject, ResponseBody as CompactResponseBody,
};
use crate::openai::compact_response::types as cpt;
use crate::openai::count_tokens::types as ot;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::openai::model_list::claude::utils::openai_error_response_from_claude;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeCreateMessageResponse> for OpenAiCompactResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeCreateMessageResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeCreateMessageResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let mut output = Vec::new();
                let mut message_content = Vec::new();

                for (index, block) in body.content.into_iter().enumerate() {
                    match block {
                        ct::BetaContentBlock::Text(text) => {
                            if !text.text.is_empty() {
                                message_content.push(
                                    cpt::CompactedResponseMessageContent::OutputText(
                                        ot::ResponseOutputText {
                                            annotations: Vec::new(),
                                            logprobs: None,
                                            text: text.text,
                                            type_: ot::ResponseOutputTextType::OutputText,
                                        },
                                    ),
                                );
                            }
                        }
                        ct::BetaContentBlock::Thinking(thinking) => {
                            if !thinking.thinking.is_empty() {
                                message_content.push(
                                    cpt::CompactedResponseMessageContent::ReasoningText(
                                        ot::ResponseReasoningTextContent {
                                            text: thinking.thinking,
                                            type_:
                                                ot::ResponseReasoningTextContentType::ReasoningText,
                                        },
                                    ),
                                );
                            }
                        }
                        ct::BetaContentBlock::RedactedThinking(thinking) => {
                            if !thinking.data.is_empty() {
                                output.push(cpt::CompactedResponseOutputItem::CompactionItem(
                                    ot::ResponseCompactionItemParam {
                                        encrypted_content: thinking.data,
                                        type_: ot::ResponseCompactionItemType::Compaction,
                                        id: Some(format!("compaction_{index}")),
                                        created_by: None,
                                    },
                                ));
                            }
                        }
                        ct::BetaContentBlock::ToolUse(tool_use) => {
                            output.push(cpt::CompactedResponseOutputItem::FunctionToolCall(
                                ot::ResponseFunctionToolCall {
                                    arguments: serde_json::to_string(&tool_use.input)
                                        .unwrap_or_else(|_| "{}".to_string()),
                                    call_id: tool_use.id.clone(),
                                    name: tool_use.name,
                                    type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                    id: Some(tool_use.id),
                                    status: Some(ot::ResponseItemStatus::Completed),
                                },
                            ));
                        }
                        ct::BetaContentBlock::ServerToolUse(tool_use) => {
                            output.push(cpt::CompactedResponseOutputItem::FunctionToolCall(
                                ot::ResponseFunctionToolCall {
                                    arguments: serde_json::to_string(&tool_use.input)
                                        .unwrap_or_else(|_| "{}".to_string()),
                                    call_id: tool_use.id.clone(),
                                    name: match tool_use.name {
                                        BetaServerToolUseName::WebSearch => "web_search",
                                        BetaServerToolUseName::WebFetch => "web_fetch",
                                        BetaServerToolUseName::CodeExecution => "code_execution",
                                        BetaServerToolUseName::BashCodeExecution => {
                                            "bash_code_execution"
                                        }
                                        BetaServerToolUseName::TextEditorCodeExecution => {
                                            "text_editor_code_execution"
                                        }
                                        BetaServerToolUseName::ToolSearchToolRegex => {
                                            "tool_search_tool_regex"
                                        }
                                        BetaServerToolUseName::ToolSearchToolBm25 => {
                                            "tool_search_tool_bm25"
                                        }
                                    }
                                    .to_string(),
                                    type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                    id: Some(tool_use.id),
                                    status: Some(ot::ResponseItemStatus::Completed),
                                },
                            ));
                        }
                        ct::BetaContentBlock::McpToolUse(tool_use) => {
                            output.push(cpt::CompactedResponseOutputItem::FunctionToolCall(
                                ot::ResponseFunctionToolCall {
                                    arguments: serde_json::to_string(&tool_use.input)
                                        .unwrap_or_else(|_| "{}".to_string()),
                                    call_id: tool_use.id.clone(),
                                    name: tool_use.name,
                                    type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                    id: Some(tool_use.id),
                                    status: Some(ot::ResponseItemStatus::Completed),
                                },
                            ));
                        }
                        ct::BetaContentBlock::Compaction(compaction) => {
                            output.push(cpt::CompactedResponseOutputItem::CompactionItem(
                                ot::ResponseCompactionItemParam {
                                    encrypted_content: compaction.content.unwrap_or_default(),
                                    type_: ot::ResponseCompactionItemType::Compaction,
                                    id: Some(format!("compaction_{index}")),
                                    created_by: None,
                                },
                            ));
                        }
                        other => {
                            message_content.push(cpt::CompactedResponseMessageContent::Text(
                                cpt::CompactedResponseTextContent {
                                    text: format!("{other:?}"),
                                    type_: cpt::CompactedResponseTextContentType::Text,
                                },
                            ));
                        }
                    }
                }

                if !message_content.is_empty() {
                    let status = match body.stop_reason {
                        Some(
                            ct::BetaStopReason::MaxTokens
                            | ct::BetaStopReason::Refusal
                            | ct::BetaStopReason::ModelContextWindowExceeded,
                        ) => ot::ResponseItemStatus::Incomplete,
                        _ => ot::ResponseItemStatus::Completed,
                    };
                    output.push(cpt::CompactedResponseOutputItem::Message(
                        cpt::CompactedResponseMessage {
                            id: format!("{}_message_0", body.id),
                            content: message_content,
                            role: cpt::CompactedResponseMessageRole::Assistant,
                            status,
                            type_: cpt::CompactedResponseMessageType::Message,
                        },
                    ));
                }

                OpenAiCompactResponse::Success {
                    stats_code,
                    headers: OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: CompactResponseBody {
                        id: body.id,
                        created_at: 0,
                        object: OpenAiCompactedResponseObject::ResponseCompaction,
                        output,
                        usage: {
                            let input_tokens = body
                                .usage
                                .input_tokens
                                .saturating_add(body.usage.cache_creation_input_tokens)
                                .saturating_add(body.usage.cache_read_input_tokens);
                            cpt::ResponseUsage {
                                input_tokens,
                                input_tokens_details: cpt::ResponseInputTokensDetails {
                                    cached_tokens: body.usage.cache_read_input_tokens,
                                },
                                output_tokens: body.usage.output_tokens,
                                output_tokens_details: cpt::ResponseOutputTokensDetails {
                                    reasoning_tokens: 0,
                                },
                                total_tokens: input_tokens.saturating_add(body.usage.output_tokens),
                            }
                        },
                    },
                }
            }
            ClaudeCreateMessageResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCompactResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_claude(stats_code, body),
            },
        })
    }
}
