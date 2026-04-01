use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::batch_embed_contents::types::GeminiContentEmbedding;
use crate::gemini::types::{GeminiApiErrorResponse, GeminiResponseHeaders};

/// Successful response body for `models.batchEmbedContents`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseBody {
    /// Embeddings for each input request, preserving order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeddings: Vec<GeminiContentEmbedding>,
}

/// Full HTTP response for Gemini `models.batchEmbedContents` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiBatchEmbedContentsResponse {
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
