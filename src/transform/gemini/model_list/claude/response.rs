use crate::claude::model_list::response::ClaudeModelListResponse;
use crate::gemini::model_list::response::{
    GeminiModelListResponse, ResponseBody as GeminiModelListResponseBody,
};
use crate::gemini::types::GeminiResponseHeaders;
use crate::transform::gemini::model_list::claude::utils::{
    gemini_error_response_from_claude, gemini_model_info_from_claude_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelListResponse> for GeminiModelListResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeModelListResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeModelListResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let next_page_token = if body.has_more && !body.last_id.is_empty() {
                    Some(body.last_id.clone())
                } else {
                    None
                };

                GeminiModelListResponse::Success {
                    stats_code,
                    headers: GeminiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: GeminiModelListResponseBody {
                        models: body
                            .data
                            .into_iter()
                            .map(gemini_model_info_from_claude_model)
                            .collect::<Vec<_>>(),
                        next_page_token,
                    },
                }
            }
            ClaudeModelListResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiModelListResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_error_response_from_claude(stats_code, body),
            },
        })
    }
}
