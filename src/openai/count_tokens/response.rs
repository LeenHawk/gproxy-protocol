use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::count_tokens::types::{OpenAiApiErrorResponse, OpenAiResponseHeaders};

/// Successful body returned by count-tokens endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseBody {
    pub input_tokens: u64,
    pub object: OpenAiCountTokensObject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiCountTokensObject {
    #[serde(rename = "response.input_tokens")]
    ResponseInputTokens,
}

/// Full HTTP response for OpenAI `responses.input_tokens.count` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiCountTokensResponse {
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
