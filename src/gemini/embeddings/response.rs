use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::embeddings::types::GeminiContentEmbedding;
use crate::gemini::types::{GeminiApiErrorResponse, GeminiResponseHeaders};

/// Successful response body for `models.embedContent`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// Generated embedding vector.
    pub embedding: GeminiContentEmbedding,
}

/// Full HTTP response for Gemini `models.embedContent` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiEmbedContentResponse {
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
