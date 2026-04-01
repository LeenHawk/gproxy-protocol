use crate::gemini::model_list::response::GeminiModelListResponse;
use crate::openai::model_list::response::OpenAiModelListResponse;
use crate::openai::types::{OpenAiListObject, OpenAiModelList, OpenAiResponseHeaders};
use crate::transform::openai::model_list::gemini::utils::{
    openai_error_response_from_gemini, openai_model_from_gemini_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelListResponse> for OpenAiModelListResponse {
    type Error = TransformError;

    fn try_from(value: GeminiModelListResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiModelListResponse::Success {
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
                        .models
                        .into_iter()
                        .map(openai_model_from_gemini_model)
                        .collect::<Vec<_>>(),
                    object: OpenAiListObject::List,
                },
            },
            GeminiModelListResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiModelListResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
