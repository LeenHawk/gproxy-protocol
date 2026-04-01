use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::video_get::types::{OpenAiApiErrorResponse, OpenAiResponseHeaders, OpenAiVideo};

/// Successful body for OpenAI `videos.retrieve` endpoint.
pub type ResponseBody = OpenAiVideo;

/// Full HTTP response for OpenAI `videos.retrieve` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiVideoGetResponse {
    Success {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: OpenAiResponseHeaders,
        /// Successful body.
        body: ResponseBody,
    },
    Error {
        /// HTTP status code returned by server (typically non-2xx).
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: OpenAiResponseHeaders,
        /// Error body.
        body: OpenAiApiErrorResponse,
    },
}
