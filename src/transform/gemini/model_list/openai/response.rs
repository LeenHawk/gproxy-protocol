use crate::gemini::model_list::response::{
    GeminiModelListResponse, ResponseBody as GeminiModelListResponseBody,
};
use crate::gemini::types::GeminiResponseHeaders;
use crate::openai::model_list::response::OpenAiModelListResponse;
use crate::transform::gemini::model_list::openai::utils::{
    gemini_error_response_from_openai, gemini_model_info_from_openai_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelListResponse> for GeminiModelListResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiModelListResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiModelListResponse::Success {
                stats_code,
                headers,
                body,
            } => GeminiModelListResponse::Success {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: GeminiModelListResponseBody {
                    models: body
                        .data
                        .into_iter()
                        .map(gemini_model_info_from_openai_model)
                        .collect::<Vec<_>>(),
                    next_page_token: None,
                },
            },
            OpenAiModelListResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiModelListResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_error_response_from_openai(stats_code, body),
            },
        })
    }
}
