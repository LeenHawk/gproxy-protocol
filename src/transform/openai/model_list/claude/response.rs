use crate::claude::model_list::response::ClaudeModelListResponse;
use crate::openai::model_list::response::OpenAiModelListResponse;
use crate::openai::types::{OpenAiListObject, OpenAiModelList, OpenAiResponseHeaders};
use crate::transform::openai::model_list::claude::utils::{
    openai_error_response_from_claude, openai_model_from_claude_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelListResponse> for OpenAiModelListResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeModelListResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeModelListResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiModelListResponse::Success {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: OpenAiModelList {
                    data: body
                        .data
                        .into_iter()
                        .map(openai_model_from_claude_model)
                        .collect::<Vec<_>>(),
                    object: OpenAiListObject::List,
                },
            },
            ClaudeModelListResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiModelListResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_claude(stats_code, body),
            },
        })
    }
}
