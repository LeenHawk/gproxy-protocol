use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::video_content_get::types::{
    GeminiApiErrorResponse, GeminiResponseHeaders, GeminiVideoContentBody,
};

/// Successful body for Veo generated content download.
pub type ResponseBody = GeminiVideoContentBody;

/// Full HTTP response for Veo generated content download.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiVideoContentGetResponse {
    Success {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
        body: ResponseBody,
    },
    Error {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
        body: GeminiApiErrorResponse,
    },
}
