use serde::{Deserialize, Serialize};

pub use crate::openai::create_response::stream::ResponseStreamEvent;
pub use crate::openai::create_response::types::{
    HttpMethod, JsonObject, Metadata, OpenAiApiError, OpenAiApiErrorResponse,
    OpenAiResponseHeaders, ResponseInput,
};
use crate::openai::create_response::{request, stream};

/// Additional metadata attached to websocket frames.
pub type OpenAiCreateResponseWebSocketClientMetadata = Metadata;

/// Payload of `response.create` websocket frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiCreateResponseCreateWebSocketRequestBody {
    /// Same body schema as `POST /responses`.
    #[serde(flatten)]
    pub request: request::RequestBody,
    /// When false, server prepares state but does not generate output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate: Option<bool>,
    /// Client-side metadata map forwarded upstream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<OpenAiCreateResponseWebSocketClientMetadata>,
}

/// Payload of `response.append` websocket frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateResponseAppendWebSocketRequestBody {
    pub input: ResponseInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<OpenAiCreateResponseWebSocketClientMetadata>,
}

/// Client frame union for Responses websocket.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum OpenAiCreateResponseWebSocketClientMessage {
    #[serde(rename = "response.create")]
    ResponseCreate(OpenAiCreateResponseCreateWebSocketRequestBody),
    #[serde(rename = "response.append")]
    ResponseAppend(OpenAiCreateResponseAppendWebSocketRequestBody),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiCreateResponseWebSocketWrappedErrorEventType {
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiCreateResponseWebSocketWrappedError {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// Provider-specific fields (for example plan type / resets_at).
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Wrapped websocket error event emitted as top-level `type=error` frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateResponseWebSocketWrappedErrorEvent {
    #[serde(rename = "type")]
    pub type_: OpenAiCreateResponseWebSocketWrappedErrorEventType,
    #[serde(
        alias = "status_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OpenAiCreateResponseWebSocketWrappedError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiCreateResponseWebSocketRateLimitEventType {
    #[serde(rename = "codex.rate_limits")]
    CodexRateLimits,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateResponseWebSocketRateLimitWindow {
    pub used_percent: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_minutes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiCreateResponseWebSocketRateLimitDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<OpenAiCreateResponseWebSocketRateLimitWindow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<OpenAiCreateResponseWebSocketRateLimitWindow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiCreateResponseWebSocketCredits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_credits: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlimited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
}

/// Codex-specific rate limit update emitted over websocket.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateResponseWebSocketRateLimitEvent {
    #[serde(rename = "type")]
    pub type_: OpenAiCreateResponseWebSocketRateLimitEventType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<OpenAiCreateResponseWebSocketRateLimitDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credits: Option<OpenAiCreateResponseWebSocketCredits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metered_limit_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_name: Option<String>,
}

/// Marker frame emitted by some gateways to signal stream completion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiCreateResponseWebSocketDoneMarker {
    #[serde(rename = "[DONE]")]
    Done,
}

/// Parsed websocket server message union.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum OpenAiCreateResponseWebSocketServerMessage {
    WrappedError(OpenAiCreateResponseWebSocketWrappedErrorEvent),
    RateLimit(OpenAiCreateResponseWebSocketRateLimitEvent),
    StreamEvent(stream::ResponseStreamEvent),
    ApiError(OpenAiApiErrorResponse),
    Done(OpenAiCreateResponseWebSocketDoneMarker),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        OpenAiCreateResponseCreateWebSocketRequestBody, OpenAiCreateResponseWebSocketClientMessage,
        OpenAiCreateResponseWebSocketServerMessage,
    };
    use crate::openai::create_response::request;

    #[test]
    fn response_create_frame_flattens_http_body() {
        let frame = OpenAiCreateResponseWebSocketClientMessage::ResponseCreate(
            OpenAiCreateResponseCreateWebSocketRequestBody {
                request: request::RequestBody {
                    model: Some("gpt-5.3-codex".to_string()),
                    stream: Some(true),
                    ..Default::default()
                },
                generate: Some(false),
                client_metadata: Some(
                    [("x-codex-turn-metadata".to_string(), "turn-1".to_string())]
                        .into_iter()
                        .collect(),
                ),
            },
        );

        let value = serde_json::to_value(frame).expect("frame should serialize");
        assert_eq!(value["type"], json!("response.create"));
        assert_eq!(value["model"], json!("gpt-5.3-codex"));
        assert_eq!(value["stream"], json!(true));
        assert_eq!(value["generate"], json!(false));
        assert_eq!(
            value["client_metadata"]["x-codex-turn-metadata"],
            json!("turn-1")
        );
        assert!(value.get("request").is_none());
    }

    #[test]
    fn websocket_wrapped_error_accepts_status_code_alias() {
        let value = json!({
            "type": "error",
            "status_code": 429,
            "error": {
                "type": "usage_limit_reached",
                "code": "websocket_connection_limit_reached",
                "message": "limit reached"
            },
            "headers": {
                "x-codex-primary-used-percent": "100"
            }
        });

        let message: OpenAiCreateResponseWebSocketServerMessage =
            serde_json::from_value(value).expect("wrapped error should parse");

        match message {
            OpenAiCreateResponseWebSocketServerMessage::WrappedError(event) => {
                assert_eq!(event.status, Some(429));
                let error = event.error.expect("error payload should exist");
                assert_eq!(
                    error.code.as_deref(),
                    Some("websocket_connection_limit_reached")
                );
            }
            _ => panic!("expected wrapped websocket error variant"),
        }
    }
}
