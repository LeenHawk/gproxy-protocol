use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// HTTP method used by generated request descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// Common response headers returned by OpenAI endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiResponseHeaders {
    /// Additional response headers.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Serde helpers for `http::StatusCode` as numeric code (e.g. 200, 404).
pub mod status_code_serde {
    use http::StatusCode;
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(value.as_u16())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer)?;
        StatusCode::from_u16(code).map_err(D::Error::custom)
    }
}

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiModel {
    /// The model identifier.
    pub id: String,
    /// The Unix timestamp (in seconds) when the model was created.
    pub created: u64,
    /// The object type, always `model`.
    pub object: OpenAiModelObject,
    /// The organization that owns the model.
    pub owned_by: String,
}

/// OpenAI model object discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiModelObject {
    #[serde(rename = "model")]
    Model,
}

/// Response body for OpenAI `models.list`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiModelList {
    /// List of model records.
    pub data: Vec<OpenAiModel>,
    /// The object type, always `list`.
    pub object: OpenAiListObject,
}

/// OpenAI list object discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiListObject {
    #[serde(rename = "list")]
    List,
}

/// Top-level OpenAI API error response wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiApiErrorResponse {
    pub error: OpenAiApiError,
}

/// Standard OpenAI API error payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiApiError {
    /// Human-readable message describing the error.
    pub message: String,
    /// Machine-readable error type.
    #[serde(rename = "type")]
    pub type_: String,
    /// Parameter related to the error, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// Error code value, if provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
