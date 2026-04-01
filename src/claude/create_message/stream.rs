use serde::{Deserialize, Serialize};

use crate::claude::create_message::types::{
    BetaContainer, BetaContentBlock, BetaContextManagementResponse, BetaIterationsUsage,
    BetaMessage, BetaServerToolUsage, BetaStopReason, BetaTextCitation, JsonObject,
};
use crate::claude::types::BetaError;

/// Parsed SSE stream body for Claude `messages.create` with `stream=true`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ClaudeCreateMessageSseStreamBody {
    /// SSE events in receive order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<ClaudeCreateMessageStreamEvent>,
}

/// Stream event payload for Claude Messages SSE responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeCreateMessageStreamEvent {
    MessageStart(BetaRawMessageStartEvent),
    ContentBlockStart(BetaRawContentBlockStartEvent),
    ContentBlockDelta(BetaRawContentBlockDeltaEvent),
    ContentBlockStop(BetaRawContentBlockStopEvent),
    MessageDelta(BetaRawMessageDeltaEvent),
    MessageStop(BetaRawMessageStopEvent),
    Ping(BetaPingEvent),
    Error(BetaStreamErrorEvent),
    Unknown(BetaUnknownStreamEvent),
}

impl From<Vec<ClaudeCreateMessageStreamEvent>> for ClaudeCreateMessageSseStreamBody {
    fn from(events: Vec<ClaudeCreateMessageStreamEvent>) -> Self {
        Self { events }
    }
}

impl From<ClaudeCreateMessageSseStreamBody> for Vec<ClaudeCreateMessageStreamEvent> {
    fn from(value: ClaudeCreateMessageSseStreamBody) -> Self {
        value.events
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaRawMessageStartEvent {
    pub message: BetaMessage,
    #[serde(rename = "type")]
    pub type_: BetaRawMessageStartEventType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaRawMessageStartEventType {
    #[serde(rename = "message_start")]
    MessageStart,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaRawContentBlockStartEvent {
    pub content_block: BetaContentBlock,
    pub index: u64,
    #[serde(rename = "type")]
    pub type_: BetaRawContentBlockStartEventType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaRawContentBlockStartEventType {
    #[serde(rename = "content_block_start")]
    ContentBlockStart,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaRawContentBlockDeltaEvent {
    pub delta: BetaRawContentBlockDelta,
    pub index: u64,
    #[serde(rename = "type")]
    pub type_: BetaRawContentBlockDeltaEventType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaRawContentBlockDeltaEventType {
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaRawContentBlockDelta {
    Text(BetaTextDelta),
    InputJson(BetaInputJsonDelta),
    Citations(BetaCitationsDelta),
    Thinking(BetaThinkingDelta),
    Signature(BetaSignatureDelta),
    Compaction(BetaCompactionContentBlockDelta),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaTextDelta {
    pub text: String,
    #[serde(rename = "type")]
    pub type_: BetaTextDeltaType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaTextDeltaType {
    #[serde(rename = "text_delta")]
    TextDelta,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaInputJsonDelta {
    pub partial_json: String,
    #[serde(rename = "type")]
    pub type_: BetaInputJsonDeltaType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaInputJsonDeltaType {
    #[serde(rename = "input_json_delta")]
    InputJsonDelta,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaCitationsDelta {
    pub citation: BetaTextCitation,
    #[serde(rename = "type")]
    pub type_: BetaCitationsDeltaType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaCitationsDeltaType {
    #[serde(rename = "citations_delta")]
    CitationsDelta,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaThinkingDelta {
    pub thinking: String,
    #[serde(rename = "type")]
    pub type_: BetaThinkingDeltaType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaThinkingDeltaType {
    #[serde(rename = "thinking_delta")]
    ThinkingDelta,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaSignatureDelta {
    pub signature: String,
    #[serde(rename = "type")]
    pub type_: BetaSignatureDeltaType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaSignatureDeltaType {
    #[serde(rename = "signature_delta")]
    SignatureDelta,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCompactionContentBlockDelta {
    /// Summary of compacted content; `null` when compaction failed.
    pub content: Option<String>,
    #[serde(rename = "type")]
    pub type_: BetaCompactionContentBlockDeltaType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaCompactionContentBlockDeltaType {
    #[serde(rename = "compaction_delta")]
    CompactionDelta,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaRawContentBlockStopEvent {
    pub index: u64,
    #[serde(rename = "type")]
    pub type_: BetaRawContentBlockStopEventType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaRawContentBlockStopEventType {
    #[serde(rename = "content_block_stop")]
    ContentBlockStop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaRawMessageDeltaEvent {
    /// Context management report; present and can be `null`.
    #[serde(default)]
    pub context_management: Option<BetaContextManagementResponse>,
    pub delta: BetaRawMessageDelta,
    #[serde(rename = "type")]
    pub type_: BetaRawMessageDeltaEventType,
    pub usage: BetaMessageDeltaUsage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BetaRawMessageDelta {
    /// Container information; can be `null`.
    #[serde(default)]
    pub container: Option<BetaContainer>,
    /// Stop reason; can be `null`.
    #[serde(default)]
    pub stop_reason: Option<BetaStopReason>,
    /// Stop sequence; can be `null`.
    #[serde(default)]
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaRawMessageDeltaEventType {
    #[serde(rename = "message_delta")]
    MessageDelta,
}

/// Cumulative usage counters carried in stream `message_delta` events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BetaMessageDeltaUsage {
    /// The cumulative number of input tokens used to create the cache entry.
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u64>,
    /// The cumulative number of input tokens read from the cache.
    #[serde(default)]
    pub cache_read_input_tokens: Option<u64>,
    /// The cumulative number of input tokens used.
    #[serde(default)]
    pub input_tokens: Option<u64>,
    /// Per-iteration usage breakdown.
    #[serde(default)]
    pub iterations: Option<BetaIterationsUsage>,
    /// The cumulative number of output tokens used.
    pub output_tokens: u64,
    /// The number of server tool requests.
    #[serde(default)]
    pub server_tool_use: Option<BetaServerToolUsage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaRawMessageStopEvent {
    #[serde(rename = "type")]
    pub type_: BetaRawMessageStopEventType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaRawMessageStopEventType {
    #[serde(rename = "message_stop")]
    MessageStop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaPingEvent {
    #[serde(rename = "type")]
    pub type_: BetaPingEventType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaPingEventType {
    #[serde(rename = "ping")]
    Ping,
}

/// Error event emitted inline in SSE stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaStreamErrorEvent {
    pub error: BetaError,
    #[serde(rename = "type")]
    pub type_: BetaStreamErrorEventType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaStreamErrorEventType {
    #[serde(rename = "error")]
    Error,
}

/// Catch-all event for future unknown stream event types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaUnknownStreamEvent {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}
