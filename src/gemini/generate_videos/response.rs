use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::generate_videos::types::{
    GeminiApiError, GeminiApiErrorResponse, GeminiGenerateVideosOperationResult,
    GeminiResponseHeaders, GeminiVideoOperationMetadata,
};

/// Successful response body for Gemini `models.generateVideos`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseBody {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GeminiVideoOperationMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<GeminiGenerateVideosOperationResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<GeminiApiError>,
}

/// Full HTTP response for Gemini `models.generateVideos` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiGenerateVideosResponse {
    Success {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
        body: Box<ResponseBody>,
    },
    Error {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
        body: GeminiApiErrorResponse,
    },
}
