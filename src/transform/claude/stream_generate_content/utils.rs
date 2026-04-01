use crate::claude::count_tokens::types::{BetaThinkingBlockType, BetaToolUseBlockType};
use crate::claude::create_message::stream::{
    BetaMessageDeltaUsage, BetaRawContentBlockDelta, BetaRawMessageDelta, ClaudeStreamEvent,
};
use crate::claude::create_message::types::{
    BetaContentBlock, BetaMessage, BetaMessageRole, BetaMessageType, BetaServiceTier,
    BetaStopReason, BetaTextBlock, BetaTextBlockType, BetaThinkingBlock, BetaToolUseBlock, Model,
};
use crate::claude::types::{BetaApiError, BetaApiErrorType, BetaError};
use crate::transform::claude::generate_content::utils::beta_usage_from_counts;

pub fn message_start_event(
    id: String,
    model: String,
    service_tier: BetaServiceTier,
    input_tokens: u64,
    cached_input_tokens: u64,
) -> ClaudeStreamEvent {
    ClaudeStreamEvent::MessageStart {
        message: BetaMessage {
            id,
            container: None,
            content: Vec::new(),
            context_management: None,
            model: Model::Custom(model),
            role: BetaMessageRole::Assistant,
            stop_reason: None,
            stop_sequence: None,
            type_: BetaMessageType::Message,
            usage: beta_usage_from_counts(input_tokens, cached_input_tokens, 0, service_tier),
        },
    }
}

pub fn start_text_block_event(index: u64) -> ClaudeStreamEvent {
    ClaudeStreamEvent::ContentBlockStart {
        content_block: BetaContentBlock::Text(BetaTextBlock {
            citations: None,
            text: String::new(),
            type_: BetaTextBlockType::Text,
        }),
        index,
    }
}

pub fn start_thinking_block_event(index: u64, signature: String) -> ClaudeStreamEvent {
    ClaudeStreamEvent::ContentBlockStart {
        content_block: BetaContentBlock::Thinking(BetaThinkingBlock {
            signature,
            thinking: String::new(),
            type_: BetaThinkingBlockType::Thinking,
        }),
        index,
    }
}

pub fn start_tool_use_block_event(
    index: u64,
    id: String,
    name: String,
) -> ClaudeStreamEvent {
    ClaudeStreamEvent::ContentBlockStart {
        content_block: BetaContentBlock::ToolUse(BetaToolUseBlock {
            id,
            input: Default::default(),
            name,
            type_: BetaToolUseBlockType::ToolUse,
            cache_control: None,
            caller: None,
        }),
        index,
    }
}

pub fn text_delta_event(index: u64, text: String) -> ClaudeStreamEvent {
    ClaudeStreamEvent::ContentBlockDelta {
        delta: BetaRawContentBlockDelta::Text { text },
        index,
    }
}

pub fn thinking_delta_event(index: u64, thinking: String) -> ClaudeStreamEvent {
    ClaudeStreamEvent::ContentBlockDelta {
        delta: BetaRawContentBlockDelta::Thinking { thinking },
        index,
    }
}

pub fn input_json_delta_event(index: u64, partial_json: String) -> ClaudeStreamEvent {
    ClaudeStreamEvent::ContentBlockDelta {
        delta: BetaRawContentBlockDelta::InputJson { partial_json },
        index,
    }
}

pub fn stop_block_event(index: u64) -> ClaudeStreamEvent {
    ClaudeStreamEvent::ContentBlockStop { index }
}

pub fn message_delta_event(
    stop_reason: Option<BetaStopReason>,
    input_tokens: u64,
    cached_input_tokens: u64,
    output_tokens: u64,
) -> ClaudeStreamEvent {
    ClaudeStreamEvent::MessageDelta {
        context_management: None,
        delta: BetaRawMessageDelta {
            container: None,
            stop_reason,
            stop_sequence: None,
        },
        usage: BetaMessageDeltaUsage {
            cache_creation_input_tokens: Some(0),
            cache_read_input_tokens: Some(cached_input_tokens),
            input_tokens: Some(input_tokens),
            iterations: None,
            output_tokens,
            server_tool_use: None,
        },
    }
}

pub fn message_stop_event() -> ClaudeStreamEvent {
    ClaudeStreamEvent::MessageStop {}
}

pub fn stream_error_event(message: String) -> ClaudeStreamEvent {
    ClaudeStreamEvent::Error {
        error: BetaError::Api(BetaApiError {
            message,
            type_: BetaApiErrorType::ApiError,
        }),
    }
}

pub fn push_text_block(
    out: &mut Vec<ClaudeStreamEvent>,
    next_block_index: &mut u64,
    text: String,
) -> bool {
    if text.is_empty() {
        return false;
    }
    let block_index = *next_block_index;
    *next_block_index = next_block_index.saturating_add(1);
    out.push(start_text_block_event(block_index));
    out.push(text_delta_event(block_index, text));
    out.push(stop_block_event(block_index));
    true
}

pub fn push_thinking_block(
    out: &mut Vec<ClaudeStreamEvent>,
    next_block_index: &mut u64,
    signature: String,
    thinking: String,
) -> bool {
    if thinking.is_empty() {
        return false;
    }
    let block_index = *next_block_index;
    *next_block_index = next_block_index.saturating_add(1);
    out.push(start_thinking_block_event(block_index, signature));
    out.push(thinking_delta_event(block_index, thinking));
    out.push(stop_block_event(block_index));
    true
}

pub fn push_tool_use_block(
    out: &mut Vec<ClaudeStreamEvent>,
    next_block_index: &mut u64,
    id: String,
    name: String,
    input_json: Option<String>,
) -> u64 {
    let block_index = *next_block_index;
    *next_block_index = next_block_index.saturating_add(1);
    out.push(start_tool_use_block_event(block_index, id, name));
    if let Some(input_json) = input_json
        && !input_json.is_empty()
    {
        out.push(input_json_delta_event(block_index, input_json));
    }
    out.push(stop_block_event(block_index));
    block_index
}
