use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gemini::generate_videos::types::{
    GeminiGenerateVideosInstance, GeminiGenerateVideosParameters, HttpMethod,
};

/// Request descriptor for Gemini `models.generateVideos` long-running endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiGenerateVideosRequest {
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

impl Default for GeminiGenerateVideosRequest {
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

/// Proxy-side request model does not carry auth query parameters.
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
    pub instances: Vec<GeminiGenerateVideosInstance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<GeminiGenerateVideosParameters>,
}
