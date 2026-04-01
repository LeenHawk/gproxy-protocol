use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::claude::create_message::types::BetaMessage;
use crate::claude::types::{BetaErrorResponse, ClaudeResponseHeaders};

/// Successful body for Claude "Create a Message" endpoint.
pub type ResponseBody = BetaMessage;

/// Full HTTP response for Claude "Create a Message" endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ClaudeCreateMessageResponse {
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
