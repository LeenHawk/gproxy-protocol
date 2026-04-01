use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::create_video::types::{
    OpenAiApiErrorResponse, OpenAiResponseHeaders, OpenAiVideo,
};

/// Successful body for OpenAI `videos.create` endpoint.
pub type ResponseBody = OpenAiVideo;

/// Full HTTP response for OpenAI `videos.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiCreateVideoResponse {
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
