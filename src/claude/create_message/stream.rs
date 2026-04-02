use serde::{Deserialize, Serialize};

use crate::claude::create_message::types::{
    BetaContainer, BetaContentBlock, BetaContextManagementResponse, BetaIterationsUsage,
    BetaMessage, BetaServerToolUsage, BetaStopReason, BetaTextCitation,
};
use crate::claude::types::BetaError;

/// Stream event payload for Claude Messages SSE responses.
///
/// Each SSE `data:` line deserializes to one of these variants,
/// discriminated by the `type` field in JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClaudeStreamEvent {
    #[serde(rename = "message_start")]
    MessageStart { message: BetaMessage },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        content_block: BetaContentBlock,
        index: u64,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        delta: BetaRawContentBlockDelta,
        index: u64,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: u64 },
    #[serde(rename = "message_delta")]
    MessageDelta {
        #[serde(default)]
        context_management: Option<BetaContextManagementResponse>,
        delta: BetaRawMessageDelta,
        usage: BetaMessageDeltaUsage,
    },
    #[serde(rename = "message_stop")]
    MessageStop {},
    #[serde(rename = "ping")]
    Ping {},
    #[serde(rename = "error")]
    Error { error: BetaError },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BetaRawContentBlockDelta {
    #[serde(rename = "text_delta")]
    Text { text: String },
    #[serde(rename = "input_json_delta")]
    InputJson { partial_json: String },
    #[serde(rename = "citations_delta")]
    Citations { citation: BetaTextCitation },
    #[serde(rename = "thinking_delta")]
    Thinking { thinking: String },
    #[serde(rename = "signature_delta")]
    Signature { signature: String },
    #[serde(rename = "compaction_delta")]
    Compaction { content: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BetaRawMessageDelta {
    #[serde(default)]
    pub container: Option<BetaContainer>,
    #[serde(default)]
    pub stop_reason: Option<BetaStopReason>,
    #[serde(default)]
    pub stop_sequence: Option<String>,
}

/// Cumulative usage counters carried in stream `message_delta` events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BetaMessageDeltaUsage {
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u64>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u64>,
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub iterations: Option<BetaIterationsUsage>,
    pub output_tokens: u64,
    #[serde(default)]
    pub server_tool_use: Option<BetaServerToolUsage>,
}
