use serde::{Deserialize, Serialize};

use crate::openai::create_chat_completions::types::{
    ChatCompletionAnnotation, ChatCompletionDeltaRole, ChatCompletionFinishReason,
    ChatCompletionFunctionCall, ChatCompletionLogprobs, ChatCompletionReasoningDetail,
    ChatCompletionServiceTier, CompletionUsage,
};

/// Streamed chat completion chunk.
///
/// Each SSE `data:` line (except `[DONE]`) deserializes to this type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub choices: Vec<ChatCompletionChunkChoice>,
    pub created: u64,
    pub model: String,
    pub object: ChatCompletionChunkObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ChatCompletionServiceTier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<CompletionUsage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionChunkObject {
    #[serde(rename = "chat.completion.chunk")]
    ChatCompletionChunk,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionChunkChoice {
    pub delta: ChatCompletionChunkDelta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<ChatCompletionFinishReason>,
    pub index: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatCompletionLogprobs>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatCompletionChunkDelta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_details: Option<Vec<ChatCompletionReasoningDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCallDelta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ChatCompletionDeltaRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<ChatCompletionAnnotation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionChunkDeltaToolCall>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatCompletionFunctionCallDelta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<ChatCompletionFunctionCall> for ChatCompletionFunctionCallDelta {
    fn from(value: ChatCompletionFunctionCall) -> Self {
        Self {
            arguments: Some(value.arguments),
            name: Some(value.name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionChunkDeltaToolCall {
    pub index: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<ChatCompletionFunctionCallDelta>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ChatCompletionChunkDeltaToolCallType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionChunkDeltaToolCallType {
    #[serde(rename = "function")]
    Function,
}
