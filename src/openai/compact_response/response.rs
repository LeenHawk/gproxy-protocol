use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::compact_response::types::{
    CompactedResponseOutputItem, OpenAiApiErrorResponse, OpenAiResponseHeaders, ResponseUsage,
};

/// Successful body returned by `responses.compact`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// Unique ID of the compacted response.
    pub id: String,
    /// Unix timestamp (seconds) when compaction finished.
    pub created_at: u64,
    /// Object discriminator (always `response.compaction`).
    pub object: OpenAiCompactedResponseObject,
    /// Compacted output items.
    pub output: Vec<CompactedResponseOutputItem>,
    /// Token accounting for compaction.
    pub usage: ResponseUsage,
}

/// Object discriminator for compacted responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiCompactedResponseObject {
    #[serde(rename = "response.compaction")]
    ResponseCompaction,
}

/// Full HTTP response for OpenAI `responses.compact` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiCompactResponse {
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
