use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::claude::types::{BetaErrorResponse, ClaudeResponseHeaders, FileMetadata};

/// Successful body for Claude "List Files" endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// List of file metadata objects.
    pub data: Vec<FileMetadata>,
    /// ID of the first file in this page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Whether there are more results available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// ID of the last file in this page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
}

/// Full HTTP response for Claude "List Files" endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeFileListResponse {
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
