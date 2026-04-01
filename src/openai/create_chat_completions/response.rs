use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::create_chat_completions::types::{
    ChatCompletion, OpenAiApiErrorResponse, OpenAiResponseHeaders,
};

/// Successful body for OpenAI `chat.completions.create` endpoint.
pub type ResponseBody = ChatCompletion;

/// Full HTTP response for OpenAI `chat.completions.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiChatCompletionsResponse {
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
