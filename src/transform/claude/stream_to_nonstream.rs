use std::collections::BTreeMap;

use http::StatusCode;

use crate::claude::create_message::response::ClaudeCreateMessageResponse;
use crate::claude::create_message::stream::{BetaRawContentBlockDelta, ClaudeStreamEvent};
use crate::claude::create_message::types::{
    BetaContentBlock, BetaErrorResponse, BetaErrorResponseType, BetaMessage, BetaTextBlock,
    BetaThinkingBlock, BetaToolUseBlock, JsonObject,
};
use crate::claude::types::{BetaError, ClaudeResponseHeaders};
use crate::transform::utils::TransformError;

#[derive(Debug, Clone)]
enum PendingBlock {
    Text(BetaTextBlock),
    Thinking(BetaThinkingBlock),
    ToolUse {
        block: BetaToolUseBlock,
        input_json_buf: String,
    },
    Other(BetaContentBlock),
}

impl PendingBlock {
    fn apply_delta(&mut self, delta: BetaRawContentBlockDelta) {
        match (self, delta) {
            (Self::Text(block), BetaRawContentBlockDelta::Text { text }) => {
                block.text.push_str(&text);
            }
            (Self::Text(block), BetaRawContentBlockDelta::Citations { citation }) => {
                if let Some(citations) = block.citations.as_mut() {
                    citations.push(citation);
                } else {
                    block.citations = Some(vec![citation]);
                }
            }
            (Self::Thinking(block), BetaRawContentBlockDelta::Thinking { thinking }) => {
                block.thinking.push_str(&thinking);
            }
            (Self::Thinking(block), BetaRawContentBlockDelta::Signature { signature }) => {
                block.signature = signature;
            }
            (
                Self::ToolUse { input_json_buf, .. },
                BetaRawContentBlockDelta::InputJson { partial_json },
            ) => {
                input_json_buf.push_str(&partial_json);
            }
            (
                Self::Other(BetaContentBlock::Compaction(block)),
                BetaRawContentBlockDelta::Compaction { content },
            ) => {
                block.content = content;
            }
            _ => {}
        }
    }

    fn into_content_block(self) -> BetaContentBlock {
        match self {
            Self::Text(block) => BetaContentBlock::Text(block),
            Self::Thinking(block) => BetaContentBlock::Thinking(block),
            Self::ToolUse {
                mut block,
                input_json_buf,
            } => {
                if !input_json_buf.is_empty() {
                    block.input =
                        serde_json::from_str::<JsonObject>(&input_json_buf).unwrap_or_default();
                }
                BetaContentBlock::ToolUse(block)
            }
            Self::Other(block) => block,
        }
    }
}

fn pending_from_content_block(content_block: BetaContentBlock) -> PendingBlock {
    match content_block {
        BetaContentBlock::Text(block) => PendingBlock::Text(block),
        BetaContentBlock::Thinking(block) => PendingBlock::Thinking(block),
        BetaContentBlock::ToolUse(block) => PendingBlock::ToolUse {
            block,
            input_json_buf: String::new(),
        },
        other => PendingBlock::Other(other),
    }
}

fn status_code_from_stream_error(error: &BetaError) -> StatusCode {
    match error {
        BetaError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
        BetaError::Authentication(_) => StatusCode::UNAUTHORIZED,
        BetaError::Billing(_) => StatusCode::PAYMENT_REQUIRED,
        BetaError::Permission(_) => StatusCode::FORBIDDEN,
        BetaError::NotFound(_) => StatusCode::NOT_FOUND,
        BetaError::RateLimit(_) => StatusCode::TOO_MANY_REQUESTS,
        BetaError::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
        BetaError::Api(_) => StatusCode::INTERNAL_SERVER_ERROR,
        BetaError::Overloaded(_) => {
            StatusCode::from_u16(529).unwrap_or(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

impl TryFrom<Vec<ClaudeStreamEvent>> for ClaudeCreateMessageResponse {
    type Error = TransformError;

    fn try_from(value: Vec<ClaudeStreamEvent>) -> Result<Self, TransformError> {
        let mut message: Option<BetaMessage> = None;
        let mut open_blocks: BTreeMap<u64, PendingBlock> = BTreeMap::new();
        let mut closed_blocks: BTreeMap<u64, BetaContentBlock> = BTreeMap::new();

        for event in value {
            match event {
                ClaudeStreamEvent::MessageStart { message: msg } => {
                    if message.is_some() {
                        return Err(TransformError::not_implemented(
                            "multiple message_start events are not supported",
                        ));
                    }
                    message = Some(msg);
                }
                ClaudeStreamEvent::ContentBlockStart {
                    content_block,
                    index,
                } => {
                    open_blocks.insert(index, pending_from_content_block(content_block));
                }
                ClaudeStreamEvent::ContentBlockDelta { delta, index } => {
                    let Some(block) = open_blocks.get_mut(&index) else {
                        return Err(TransformError::not_implemented(
                            "content_block_delta received before content_block_start",
                        ));
                    };
                    block.apply_delta(delta);
                }
                ClaudeStreamEvent::ContentBlockStop { index } => {
                    let Some(block) = open_blocks.remove(&index) else {
                        return Err(TransformError::not_implemented(
                            "content_block_stop received before content_block_start",
                        ));
                    };
                    closed_blocks.insert(index, block.into_content_block());
                }
                ClaudeStreamEvent::MessageDelta {
                    context_management,
                    delta,
                    usage,
                } => {
                    let Some(message) = message.as_mut() else {
                        return Err(TransformError::not_implemented(
                            "message_delta received before message_start",
                        ));
                    };

                    if let Some(context_management) = context_management {
                        message.context_management = Some(context_management);
                    }

                    message.stop_reason = delta.stop_reason;
                    message.stop_sequence = delta.stop_sequence;
                    if let Some(container) = delta.container {
                        message.container = Some(container);
                    }

                    if let Some(input_tokens) = usage.input_tokens {
                        message.usage.input_tokens = input_tokens;
                    }
                    if let Some(cache_read_input_tokens) = usage.cache_read_input_tokens {
                        message.usage.cache_read_input_tokens = cache_read_input_tokens;
                    }
                    if let Some(cache_creation_input_tokens) = usage.cache_creation_input_tokens {
                        message.usage.cache_creation_input_tokens = cache_creation_input_tokens;
                    }
                    if let Some(iterations) = usage.iterations {
                        message.usage.iterations = iterations;
                    }
                    if let Some(server_tool_use) = usage.server_tool_use {
                        message.usage.server_tool_use = server_tool_use;
                    }
                    message.usage.output_tokens = usage.output_tokens;
                }
                ClaudeStreamEvent::MessageStop {} => {}
                ClaudeStreamEvent::Ping {} => {}
                ClaudeStreamEvent::Error { error } => {
                    return Ok(ClaudeCreateMessageResponse::Error {
                        stats_code: status_code_from_stream_error(&error),
                        headers: ClaudeResponseHeaders {
                            extra: BTreeMap::new(),
                        },
                        body: BetaErrorResponse {
                            error,
                            request_id: String::new(),
                            type_: BetaErrorResponseType::Error,
                        },
                    });
                }
            }
        }

        let Some(mut message) = message else {
            return Err(TransformError::not_implemented(
                "message_start event is required for stream_to_nonstream conversion",
            ));
        };

        for (index, block) in open_blocks {
            closed_blocks.insert(index, block.into_content_block());
        }

        if !closed_blocks.is_empty() {
            message.content = closed_blocks.into_values().collect();
        }

        Ok(ClaudeCreateMessageResponse::Success {
            stats_code: StatusCode::OK,
            headers: ClaudeResponseHeaders {
                extra: BTreeMap::new(),
            },
            body: message,
        })
    }
}
