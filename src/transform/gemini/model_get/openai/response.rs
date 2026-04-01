use crate::gemini::model_get::response::GeminiModelGetResponse;
use crate::gemini::types::GeminiResponseHeaders;
use crate::openai::model_get::response::OpenAiModelGetResponse;
use crate::transform::gemini::model_get::openai::utils::{
    gemini_error_response_from_openai, gemini_model_info_from_openai_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelGetResponse> for GeminiModelGetResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiModelGetResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiModelGetResponse::Success {
                stats_code,
                headers,
                body,
            } => GeminiModelGetResponse::Success {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_model_info_from_openai_model(body),
            },
            OpenAiModelGetResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiModelGetResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_error_response_from_openai(stats_code, body),
            },
        })
    }
}
