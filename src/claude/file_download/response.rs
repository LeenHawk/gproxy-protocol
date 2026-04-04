use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::claude::types::{BetaErrorResponse, ClaudeResponseHeaders};

/// Successful body for Claude "Download File" endpoint.
///
/// The response is raw binary file content.  We represent it as opaque bytes
/// since the MIME type varies per file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// Raw file content bytes.
    pub content: Vec<u8>,
    /// Content-Type header value returned by the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// Full HTTP response for Claude "Download File" endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeFileDownloadResponse {
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
