use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gemini::count_tokens::types::{GeminiContent, GeminiGenerateContentRequest, HttpMethod};

/// Request descriptor for Gemini `models.countTokens` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiCountTokensRequest {
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

impl Default for GeminiCountTokensRequest {
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

/// Proxy-side request model does not carry query parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Prompt input content.
    ///
    /// This is ignored when `generate_content_request` is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<GeminiContent>>,
    /// Full generation request used for counting tokens.
    ///
    /// This field is mutually exclusive with `contents`.
    #[serde(
        rename = "generateContentRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_content_request: Option<GeminiGenerateContentRequest>,
}
