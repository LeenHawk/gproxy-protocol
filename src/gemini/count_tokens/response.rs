use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::count_tokens::types::GeminiModalityTokenCount;
use crate::gemini::types::{GeminiApiErrorResponse, GeminiResponseHeaders};

/// Successful response body for Gemini `models.countTokens`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// Number of tokens in the prompt input.
    #[serde(rename = "totalTokens")]
    pub total_tokens: u64,
    /// Number of tokens in cached content, when cache is used.
    #[serde(
        rename = "cachedContentTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cached_content_token_count: Option<u64>,
    /// Per-modality token details for prompt input.
    #[serde(
        rename = "promptTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prompt_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
    /// Per-modality token details for cached input.
    #[serde(
        rename = "cacheTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
}

/// Full HTTP response for Gemini `models.countTokens` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiCountTokensResponse {
    Success {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Successful body.
        body: ResponseBody,
    },
    Error {
        /// HTTP status code returned by server (typically non-2xx).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Error body.
        body: GeminiApiErrorResponse,
    },
}
