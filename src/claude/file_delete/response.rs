use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::claude::types::{BetaErrorResponse, ClaudeResponseHeaders, DeletedFile};

/// Successful body — `DeletedFile`.
pub type ResponseBody = DeletedFile;

/// Full HTTP response for Claude "Delete File" endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeFileDeleteResponse {
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
