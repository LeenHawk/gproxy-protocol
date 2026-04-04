use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::claude::types::{AnthropicBeta, AnthropicVersion, HttpMethod};

/// Request descriptor for Claude "List Files" endpoint.
///
/// `GET /v1/files`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaudeFileListRequest {
    pub method: HttpMethod,
    pub path: PathParameters,
    pub query: QueryParameters,
    pub headers: RequestHeaders,
    pub body: RequestBody,
}

impl Default for ClaudeFileListRequest {
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
pub struct PathParameters {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {
    /// Pagination cursor: return records after this file id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
    /// Pagination cursor: return records before this file id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
    /// Number of items per page (1..=1000, default 20).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

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

/// List files request has no JSON body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestBody {}
