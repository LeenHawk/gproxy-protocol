use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gemini::live::types::{GeminiBidiGenerateContentClientMessage, HttpMethod};

/// Request descriptor for Gemini Live WebSocket endpoint (`BidiGenerateContent`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiLiveConnectRequest {
    /// HTTP method used by WebSocket handshake.
    pub method: HttpMethod,
    /// Path selector for Live RPC.
    pub path: PathParameters,
    /// Optional query parameters for authentication.
    pub query: QueryParameters,
    /// Optional HTTP headers for authentication.
    pub headers: RequestHeaders,
    /// Optional first WebSocket frame to send after connect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<RequestBody>,
}

impl Default for GeminiLiveConnectRequest {
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
    /// RPC route under `/ws/`.
    ///
    /// Default: `...GenerativeService.BidiGenerateContent`.
    #[serde(default)]
    pub rpc: GeminiLiveRpcMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GeminiLiveRpcMethod {
    #[default]
    #[serde(rename = "google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent")]
    BidiGenerateContent,
    #[serde(
        rename = "google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContentConstrained"
    )]
    BidiGenerateContentConstrained,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {
    /// API key query parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Ephemeral token query parameter for constrained endpoint.
    #[serde(
        rename = "access_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    /// Optional token header (`Token <token>`).
    #[serde(
        rename = "Authorization",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization: Option<String>,
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// A single client message frame sent over the Live WebSocket.
pub type RequestBody = GeminiBidiGenerateContentClientMessage;
