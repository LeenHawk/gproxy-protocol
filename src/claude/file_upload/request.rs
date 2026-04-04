use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::claude::types::{AnthropicBeta, AnthropicVersion, HttpMethod};

/// Request descriptor for Claude "Upload File" endpoint.
///
/// `POST /v1/files` — multipart/form-data upload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaudeFileUploadRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Multipart body descriptor.
    pub body: RequestBody,
}

impl Default for ClaudeFileUploadRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Post,
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

/// Multipart body for file upload.
///
/// The actual HTTP request uses `multipart/form-data` with a `file` part
/// and an optional `purpose` part.  This struct captures the logical fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Raw file bytes (the `file` multipart part).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file: Vec<u8>,
    /// Original filename for the `file` part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Upload purpose (e.g. `"user_data"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}
