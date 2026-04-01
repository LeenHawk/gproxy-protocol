use crate::claude::model_get::response::ClaudeModelGetResponse;
use crate::claude::types::ClaudeResponseHeaders;
use crate::openai::model_get::response::OpenAiModelGetResponse;
use crate::transform::claude::model_list::openai::utils::{
    beta_error_response_from_openai, beta_model_info_from_openai_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelGetResponse> for ClaudeModelGetResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiModelGetResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiModelGetResponse::Success {
                stats_code,
                headers,
                body,
            } => ClaudeModelGetResponse::Success {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_model_info_from_openai_model(body),
            },
            OpenAiModelGetResponse::Error {
                stats_code,
                headers,
                body,
            } => ClaudeModelGetResponse::Error {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_error_response_from_openai(stats_code, body),
            },
        })
    }
}
