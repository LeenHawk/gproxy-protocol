use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::stream_generate_content::stream::{GeminiNdjsonStreamBody, GeminiSseStreamBody};
use crate::gemini::types::{GeminiApiErrorResponse, GeminiResponseHeaders};

/// Full HTTP response for Gemini `models.streamGenerateContent` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiStreamGenerateContentResponse {
    NdjsonSuccess {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Stream payload encoded as newline-delimited JSON objects.
        body: GeminiNdjsonStreamBody,
    },
    SseSuccess {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Stream payload encoded as Server-Sent Events (`alt=sse`).
        body: GeminiSseStreamBody,
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
