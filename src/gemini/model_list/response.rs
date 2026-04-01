use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::types::{GeminiApiErrorResponse, GeminiModelInfo, GeminiResponseHeaders};

/// Successful body for Gemini `models.list` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseBody {
    /// List of models in this page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<GeminiModelInfo>,
    /// Token to fetch next page.
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
}

/// Full HTTP response for Gemini `models.list` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiModelListResponse {
    Success {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Successful body.
        body: ResponseBody,
    },
    Error {
        /// HTTP status code returned by server (typically non-2xx).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Error body.
        body: GeminiApiErrorResponse,
    },
}
