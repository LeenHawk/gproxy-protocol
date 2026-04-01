use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gemini::batch_embed_contents::types::{GeminiContent, GeminiTaskType, HttpMethod};

/// Request descriptor for Gemini `models.batchEmbedContents` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiBatchEmbedContentsRequest {
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

impl Default for GeminiBatchEmbedContentsRequest {
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

/// Batch embed request body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Embed requests for this batch.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<BatchRequestItem>,
}

/// One `EmbedContentRequest` item inside `batchEmbedContents`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestItem {
    /// Model name that must match the parent batch model.
    pub model: String,
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
