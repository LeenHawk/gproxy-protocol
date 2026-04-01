use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::types::{GeminiApiErrorResponse, GeminiResponseHeaders};

/// Full HTTP response envelope for Gemini `models.streamGenerateContent`.
///
/// The actual stream chunks (`GeminiNdjsonChunk` or `GeminiSseChunk`) are
/// processed one at a time by the transport layer — not collected here.
/// This type only captures the initial HTTP response metadata and error case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiStreamGenerateContentResponse {
    Success {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
    },
    Error {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
        body: GeminiApiErrorResponse,
    },
}
