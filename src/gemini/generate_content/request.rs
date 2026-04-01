use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gemini::generate_content::types::{
    GeminiContent, GeminiGenerationConfig, GeminiSafetySetting, GeminiTool, GeminiToolConfig,
    HttpMethod,
};

/// Request descriptor for Gemini `models.generateContent` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiGenerateContentRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body.
    pub body: RequestBody,
}

impl Default for GeminiGenerateContentRequest {
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
pub struct PathParameters {
    /// Resource name in form `models/{model}`.
    pub model: String,
}

/// Proxy-side request model does not carry query parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Conversation content for this turn.
    pub contents: Vec<GeminiContent>,
    /// Optional tool definitions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<GeminiTool>>,
    /// Optional tool configuration.
    #[serde(
        rename = "toolConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_config: Option<GeminiToolConfig>,
    /// Optional safety settings.
    #[serde(
        rename = "safetySettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub safety_settings: Option<Vec<GeminiSafetySetting>>,
    /// Optional system instruction content.
    #[serde(
        rename = "systemInstruction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_instruction: Option<GeminiContent>,
    /// Optional generation controls.
    #[serde(
        rename = "generationConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generation_config: Option<GeminiGenerationConfig>,
    /// Optional cache reference, e.g. `cachedContents/{id}`.
    #[serde(
        rename = "cachedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cached_content: Option<String>,
    /// Optional logging behavior override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
}
