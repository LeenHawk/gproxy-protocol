use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::claude::types::{BetaErrorResponse, ClaudeResponseHeaders, FileMetadata};

/// Successful body — `FileMetadata`.
pub type ResponseBody = FileMetadata;

/// Full HTTP response for Claude "Get File Metadata" endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeFileGetResponse {
    Success {
        #[serde(with = "crate::claude::types::status_code_serde")]
        stats_code: StatusCode,
        headers: ClaudeResponseHeaders,
        body: ResponseBody,
    },
    Error {
        #[serde(with = "crate::claude::types::status_code_serde")]
        stats_code: StatusCode,
        headers: ClaudeResponseHeaders,
        body: BetaErrorResponse,
    },
}
