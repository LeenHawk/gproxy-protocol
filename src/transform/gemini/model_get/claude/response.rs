use crate::claude::model_get::response::ClaudeModelGetResponse;
use crate::gemini::model_get::response::GeminiModelGetResponse;
use crate::gemini::types::GeminiResponseHeaders;
use crate::transform::gemini::model_get::claude::utils::{
    gemini_error_response_from_claude, gemini_model_info_from_claude_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelGetResponse> for GeminiModelGetResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeModelGetResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeModelGetResponse::Success {
                stats_code,
                headers,
                body,
            } => GeminiModelGetResponse::Success {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_model_info_from_claude_model(body),
            },
            ClaudeModelGetResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiModelGetResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_error_response_from_claude(stats_code, body),
            },
        })
    }
}
