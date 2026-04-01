use crate::claude::model_list::response::{
    ClaudeModelListResponse, ResponseBody as ClaudeModelListResponseBody,
};
use crate::claude::types::ClaudeResponseHeaders;
use crate::gemini::model_list::response::GeminiModelListResponse;
use crate::transform::claude::model_list::gemini::utils::{
    beta_error_response_from_gemini, beta_model_info_from_gemini_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelListResponse> for ClaudeModelListResponse {
    type Error = TransformError;

    fn try_from(value: GeminiModelListResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiModelListResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let has_more = body.next_page_token.is_some();
                let data = body
                    .models
                    .into_iter()
                    .map(beta_model_info_from_gemini_model)
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
                        has_more,
                        last_id,
                    },
                }
            }
            GeminiModelListResponse::Error {
                stats_code,
                headers,
                body,
            } => ClaudeModelListResponse::Error {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
