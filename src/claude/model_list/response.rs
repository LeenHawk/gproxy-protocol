use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::claude::types::{BetaErrorResponse, BetaModelInfo, ClaudeResponseHeaders};

/// Successful body for Claude "List Models" endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// List of model records.
    pub data: Vec<BetaModelInfo>,
    /// First id in this page.
    pub first_id: String,
    /// Whether there are more pages.
    pub has_more: bool,
    /// Last id in this page.
    pub last_id: String,
}

/// Full HTTP response for Claude "List Models" endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeModelListResponse {
    Success {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::claude::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: ClaudeResponseHeaders,
        /// Successful body.
        body: ResponseBody,
    },
    Error {
        /// HTTP status code returned by server (typically 400/401/403/404/413/429/500/529).
        #[serde(with = "crate::claude::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: ClaudeResponseHeaders,
        /// Error body.
        body: BetaErrorResponse,
    },
}
