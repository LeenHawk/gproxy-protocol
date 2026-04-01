use serde::{Deserialize, Serialize};

use crate::openai::create_chat_completions::types::{
    ChatCompletionAnnotation, ChatCompletionDeltaRole, ChatCompletionFinishReason,
    ChatCompletionFunctionCall, ChatCompletionLogprobs, ChatCompletionReasoningDetail,
    ChatCompletionServiceTier, CompletionUsage,
};

/// Parsed SSE stream body for `POST /chat/completions` with `stream=true`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiChatCompletionsSseStreamBody {
    /// SSE events in receive order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<OpenAiChatCompletionsSseEvent>,
}

/// A single SSE event frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiChatCompletionsSseEvent {
    /// Optional SSE `event` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// SSE `data` field payload.
    pub data: OpenAiChatCompletionsSseData,
}

/// SSE `data` payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum OpenAiChatCompletionsSseData {
    /// A regular stream chunk with `ChatCompletionChunk` shape.
    Chunk(ChatCompletionChunk),
    /// Stream end marker (`[DONE]`).
    Done(String),
}

impl OpenAiChatCompletionsSseData {
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Done(marker) if marker == "[DONE]")
    }
}

/// Streamed chat completion chunk.
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
    /// Optional stream obfuscation payload when enabled.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::create_chat_completions::types::ChatCompletionReasoningDetailType;

    #[test]
    fn chunk_delta_reasoning_content_roundtrip() {
        let delta = ChatCompletionChunkDelta {
            reasoning_content: Some("reasoning text".to_string()),
            ..ChatCompletionChunkDelta::default()
        };

        let value = serde_json::to_value(&delta).unwrap();
        assert_eq!(value["reasoning_content"], "reasoning text");

        let decoded: ChatCompletionChunkDelta = serde_json::from_value(value).unwrap();
        assert_eq!(decoded.reasoning_content.as_deref(), Some("reasoning text"));
    }

    #[test]
    fn chunk_delta_reasoning_details_roundtrip() {
        let delta = ChatCompletionChunkDelta {
            reasoning_details: Some(vec![ChatCompletionReasoningDetail {
                type_: ChatCompletionReasoningDetailType::ReasoningEncrypted,
                id: Some("reasoning_0".to_string()),
                data: Some("sig".to_string()),
            }]),
            ..ChatCompletionChunkDelta::default()
        };

        let value = serde_json::to_value(&delta).unwrap();
        assert_eq!(value["reasoning_details"][0]["type"], "reasoning.encrypted");
        assert_eq!(value["reasoning_details"][0]["id"], "reasoning_0");
        assert_eq!(value["reasoning_details"][0]["data"], "sig");

        let decoded: ChatCompletionChunkDelta = serde_json::from_value(value).unwrap();
        assert_eq!(
            decoded
                .reasoning_details
                .as_ref()
                .and_then(|details| details.first())
                .and_then(|detail| detail.id.as_deref()),
            Some("reasoning_0")
        );
    }
}
