use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::create_image_edit::types::{
    OpenAiApiErrorResponse, OpenAiCreateImageEditResponseBody, OpenAiResponseHeaders,
};

/// Successful body for OpenAI `images.edit` endpoint.
pub type ResponseBody = OpenAiCreateImageEditResponseBody;

/// Full HTTP response for OpenAI `images.edit` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiCreateImageEditResponse {
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
