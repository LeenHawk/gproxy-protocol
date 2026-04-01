use crate::gemini::model_get::response::GeminiModelGetResponse;
use crate::openai::model_get::response::OpenAiModelGetResponse;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::openai::model_get::gemini::utils::{
    openai_error_response_from_gemini, openai_model_from_gemini_model,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelGetResponse> for OpenAiModelGetResponse {
    type Error = TransformError;

    fn try_from(value: GeminiModelGetResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiModelGetResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiModelGetResponse::Success {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_model_from_gemini_model(body),
            },
            GeminiModelGetResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiModelGetResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
