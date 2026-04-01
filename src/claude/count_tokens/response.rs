use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::claude::count_tokens::types::BetaMessageTokensCount;
use crate::claude::types::{BetaErrorResponse, ClaudeResponseHeaders};

/// Successful body for Claude "Count Tokens" endpoint.
pub type ResponseBody = BetaMessageTokensCount;

/// Full HTTP response for Claude "Count Tokens" endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeCountTokensResponse {
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
