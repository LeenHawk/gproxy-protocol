use crate::claude::model_get::response::ClaudeModelGetResponse;
use crate::openai::model_get::response::OpenAiModelGetResponse;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::openai::model_get::claude::utils::{
    openai_error_response_from_claude, openai_model_from_claude_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelGetResponse> for OpenAiModelGetResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeModelGetResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeModelGetResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiModelGetResponse::Success {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_model_from_claude_model(body),
            },
            ClaudeModelGetResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiModelGetResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_claude(stats_code, body),
            },
        })
    }
}
