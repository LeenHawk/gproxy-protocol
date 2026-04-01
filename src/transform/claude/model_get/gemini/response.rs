use crate::claude::model_get::response::ClaudeModelGetResponse;
use crate::claude::types::ClaudeResponseHeaders;
use crate::gemini::model_get::response::GeminiModelGetResponse;
use crate::transform::claude::model_list::gemini::utils::{
    beta_error_response_from_gemini, beta_model_info_from_gemini_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelGetResponse> for ClaudeModelGetResponse {
    type Error = TransformError;

    fn try_from(value: GeminiModelGetResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiModelGetResponse::Success {
                stats_code,
                headers,
                body,
            } => ClaudeModelGetResponse::Success {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_model_info_from_gemini_model(body),
            },
            GeminiModelGetResponse::Error {
                stats_code,
                headers,
                body,
            } => ClaudeModelGetResponse::Error {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
