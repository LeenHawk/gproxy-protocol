use crate::claude::model_list::response::{
    ClaudeModelListResponse, ResponseBody as ClaudeModelListResponseBody,
};
use crate::claude::types::ClaudeResponseHeaders;
use crate::openai::model_list::response::OpenAiModelListResponse;
use crate::transform::claude::model_list::openai::utils::{
    beta_error_response_from_openai, beta_model_info_from_openai_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelListResponse> for ClaudeModelListResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiModelListResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiModelListResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let data = body
                    .data
                    .into_iter()
                    .map(beta_model_info_from_openai_model)
                    .collect::<Vec<_>>();
                let first_id = data
                    .first()
                    .map(|model| model.id.clone())
                    .unwrap_or_default();
                let last_id = data
                    .last()
                    .map(|model| model.id.clone())
                    .unwrap_or_default();

                ClaudeModelListResponse::Success {
                    stats_code,
                    headers: ClaudeResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ClaudeModelListResponseBody {
                        data,
                        first_id,
                        has_more: false,
                        last_id,
                    },
                }
            }
            OpenAiModelListResponse::Error {
                stats_code,
                headers,
                body,
            } => ClaudeModelListResponse::Error {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_error_response_from_openai(stats_code, body),
            },
        })
    }
}
