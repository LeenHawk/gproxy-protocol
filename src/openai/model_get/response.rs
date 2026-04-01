use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::types::{OpenAiApiErrorResponse, OpenAiModel, OpenAiResponseHeaders};

/// Successful body for OpenAI `models.retrieve` endpoint.
pub type ResponseBody = OpenAiModel;

/// Full HTTP response for OpenAI `models.retrieve` endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiModelGetResponse {
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
