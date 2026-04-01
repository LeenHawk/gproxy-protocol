use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::embeddings::types::{
    OpenAiApiErrorResponse, OpenAiCreateEmbeddingResponse, OpenAiResponseHeaders,
};

/// Successful body for OpenAI `embeddings.create` endpoint.
pub type ResponseBody = OpenAiCreateEmbeddingResponse;

/// Full HTTP response for OpenAI `embeddings.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiEmbeddingsResponse {
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
