use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gemini::types::HttpMethod;

/// Request descriptor for Gemini `models.streamGenerateContent` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiStreamGenerateContentRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body.
    pub body: RequestBody,
}

impl Default for GeminiStreamGenerateContentRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {
    /// Resource name in form `models/{model}`.
    pub model: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {
    /// Optional response transport selector.
    ///
    /// When omitted, server uses newline-delimited JSON (NDJSON).
    /// Set `alt=sse` to request Server-Sent Events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<AltQueryParameter>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AltQueryParameter {
    #[serde(rename = "sse")]
    Sse,
}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Stream endpoint shares the same JSON body as `models.generateContent`.
pub type RequestBody = crate::gemini::generate_content::request::RequestBody;
