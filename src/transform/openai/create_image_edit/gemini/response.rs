use crate::gemini::generate_content::response::GeminiGenerateContentResponse;
use crate::openai::create_image_edit::response::OpenAiCreateImageEditResponse;
use crate::transform::openai::create_image::gemini::utils::{
    create_image_response_body_from_gemini_response, openai_response_headers_from_gemini,
};
use crate::transform::openai::model_list::gemini::utils::openai_error_response_from_gemini;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiGenerateContentResponse> for OpenAiCreateImageEditResponse {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiGenerateContentResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiCreateImageEditResponse::Success {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: create_image_response_body_from_gemini_response(body)?,
            },
            GeminiGenerateContentResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCreateImageEditResponse::Error {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
