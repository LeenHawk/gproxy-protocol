use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gemini::embeddings::types::{GeminiContent, GeminiTaskType, HttpMethod};

/// Request descriptor for Gemini `models.embedContent` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiEmbedContentRequest {
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

impl Default for GeminiEmbedContentRequest {
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

/// Embed request body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestBody {
    /// The content to embed. Only `parts.text` is counted.
    pub content: GeminiContent,
    /// Optional embedding task type.
    #[serde(rename = "taskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<GeminiTaskType>,
    /// Optional document title, used with `RETRIEVAL_DOCUMENT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional reduced embedding dimension.
    #[serde(
        rename = "outputDimensionality",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub output_dimensionality: Option<u32>,
}

impl Default for RequestBody {
    fn default() -> Self {
        Self {
            content: GeminiContent {
                parts: Vec::new(),
                role: None,
            },
            task_type: None,
            title: None,
            output_dimensionality: None,
        }
    }
}
