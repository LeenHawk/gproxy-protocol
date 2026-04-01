use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

/// Shared JSON object map for unknown/dynamic fields.
pub type JsonObject = BTreeMap<String, Value>;

/// Common response headers returned by Gemini endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiResponseHeaders {
    /// Additional response headers.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Serde helpers for `http::StatusCode` as numeric code (e.g. 200, 400).
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

/// Information about a Gemini model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiModelInfo {
    /// Resource name of the model, e.g. `models/gemini-2.0-flash`.
    pub name: String,
    /// Base model id, e.g. `gemini-2.0-flash`.
    #[serde(
        rename = "baseModelId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_model_id: Option<String>,
    /// Major version string, e.g. `2.0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Human-readable model name.
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    /// Short description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Maximum number of input tokens allowed.
    #[serde(
        rename = "inputTokenLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub input_token_limit: Option<u64>,
    /// Maximum number of output tokens allowed.
    #[serde(
        rename = "outputTokenLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub output_token_limit: Option<u64>,
    /// Supported generation methods.
    #[serde(
        rename = "supportedGenerationMethods",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_generation_methods: Option<Vec<String>>,
    /// Whether this model supports thinking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<bool>,
    /// Default temperature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Maximum temperature.
    #[serde(
        rename = "maxTemperature",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_temperature: Option<f64>,
    /// Default nucleus sampling threshold.
    #[serde(rename = "topP", default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Default top-k value.
    #[serde(rename = "topK", default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u64>,
}

/// Google API style error envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiApiErrorResponse {
    pub error: GeminiApiError,
}

/// Google API style error object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiApiError {
    /// Numeric HTTP-like status code.
    pub code: i32,
    /// Human-readable error message.
    pub message: String,
    /// String status code such as `INVALID_ARGUMENT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Optional structured details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<JsonObject>>,
}
