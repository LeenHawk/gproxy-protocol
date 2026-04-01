use crate::gemini::video_operation_get::response::GeminiVideoOperationGetResponse;
use crate::openai::video_get::response::OpenAiVideoGetResponse;
use crate::transform::openai::create_image::gemini::utils::openai_response_headers_from_gemini;
use crate::transform::openai::create_video::utils::openai_video_from_gemini_operation;
use crate::transform::openai::model_list::gemini::utils::openai_error_response_from_gemini;

impl TryFrom<GeminiVideoOperationGetResponse> for OpenAiVideoGetResponse {
    type Error = crate::transform::utils::TransformError;

    fn try_from(
        value: GeminiVideoOperationGetResponse,
    ) -> Result<Self, crate::transform::utils::TransformError> {
        Ok(match value {
            GeminiVideoOperationGetResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiVideoGetResponse::Success {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: openai_video_from_gemini_operation(*body),
            },
            GeminiVideoOperationGetResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiVideoGetResponse::Error {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
