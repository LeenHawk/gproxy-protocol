use crate::gemini::generate_content::response::{
    GeminiGenerateContentResponse, ResponseBody as GeminiGenerateContentResponseBody,
};
use crate::openai::create_image::response::OpenAiCreateImageResponse;
use crate::openai::create_image::types as it;
use crate::transform::openai::create_image::gemini::utils::{
    create_image_response_body_from_gemini_response, openai_response_headers_from_gemini,
};
use crate::transform::openai::model_list::gemini::utils::openai_error_response_from_gemini;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiGenerateContentResponseBody> for it::OpenAiCreateImageResponseBody {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentResponseBody) -> Result<Self, TransformError> {
        create_image_response_body_from_gemini_response(value)
    }
}

impl TryFrom<GeminiGenerateContentResponse> for OpenAiCreateImageResponse {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiGenerateContentResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiCreateImageResponse::Success {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: it::OpenAiCreateImageResponseBody::try_from(body)?,
            },
            GeminiGenerateContentResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCreateImageResponse::Error {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
