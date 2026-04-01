use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::generate_videos::response::ResponseBody;
use crate::gemini::video_operation_get::types::{GeminiApiErrorResponse, GeminiResponseHeaders};

/// Successful body for Gemini Veo operation polling endpoint.
pub type ResponseBodyAlias = ResponseBody;

/// Full HTTP response for Gemini Veo operation polling endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiVideoOperationGetResponse {
    Success {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
        body: Box<ResponseBodyAlias>,
    },
    Error {
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        headers: GeminiResponseHeaders,
        body: GeminiApiErrorResponse,
    },
}
