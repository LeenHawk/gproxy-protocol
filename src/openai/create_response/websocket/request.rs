use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::create_response::websocket::types::{
    HttpMethod, OpenAiCreateResponseWebSocketClientMessage,
};

/// Request descriptor for OpenAI Responses WebSocket endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateResponseWebSocketConnectRequest {
    /// HTTP method used by WebSocket handshake.
    pub method: HttpMethod,
    /// Path selector.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Optional first WebSocket frame to send after connect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<RequestBody>,
}

impl Default for OpenAiCreateResponseWebSocketConnectRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {
    /// WebSocket route under provider base URL.
    #[serde(default)]
    pub endpoint: OpenAiCreateResponseWebSocketEndpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OpenAiCreateResponseWebSocketEndpoint {
    #[default]
    #[serde(rename = "responses")]
    Responses,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {
    /// Azure-compatible API version query key.
    #[serde(
        rename = "api-version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    /// Provider-specific passthrough query params.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(
        rename = "Authorization",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization: Option<String>,
    #[serde(
        rename = "OpenAI-Beta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub openai_beta: Option<String>,
    #[serde(
        rename = "x-codex-turn-state",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub x_codex_turn_state: Option<String>,
    #[serde(
        rename = "x-codex-turn-metadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub x_codex_turn_metadata: Option<String>,
    #[serde(
        rename = "session_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_id: Option<String>,
    #[serde(
        rename = "ChatGPT-Account-ID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chatgpt_account_id: Option<String>,
    /// Provider-specific passthrough headers.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// A single client message frame sent over Responses WebSocket.
pub type RequestBody = OpenAiCreateResponseWebSocketClientMessage;
