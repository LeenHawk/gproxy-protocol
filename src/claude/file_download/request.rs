use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::claude::types::{AnthropicBeta, AnthropicVersion, HttpMethod};

/// Request descriptor for Claude "Download File" endpoint.
///
/// `GET /v1/files/{file_id}/content`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaudeFileDownloadRequest {
    pub method: HttpMethod,
    pub path: PathParameters,
    pub query: QueryParameters,
    pub headers: RequestHeaders,
    pub body: RequestBody,
}

impl Default for ClaudeFileDownloadRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {
    /// ID of the file to download.
    pub file_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(rename = "anthropic-version")]
    pub anthropic_version: AnthropicVersion,
    #[serde(
        rename = "anthropic-beta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anthropic_beta: Option<Vec<AnthropicBeta>>,
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Download file request has no JSON body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestBody {}
