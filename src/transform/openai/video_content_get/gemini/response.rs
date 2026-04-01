use crate::gemini::video_content_get::response::GeminiVideoContentGetResponse;
use crate::openai::video_content_get::response::OpenAiVideoContentGetResponse;
use crate::openai::video_content_get::types::OpenAiVideoContentBody;
use crate::transform::openai::create_image::gemini::utils::openai_response_headers_from_gemini;
use crate::transform::openai::model_list::gemini::utils::openai_error_response_from_gemini;

impl TryFrom<GeminiVideoContentGetResponse> for OpenAiVideoContentGetResponse {
    type Error = crate::transform::utils::TransformError;

    fn try_from(
        value: GeminiVideoContentGetResponse,
    ) -> Result<Self, crate::transform::utils::TransformError> {
        Ok(match value {
            GeminiVideoContentGetResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiVideoContentGetResponse::Success {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: OpenAiVideoContentBody { bytes: body.bytes },
            },
            GeminiVideoContentGetResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiVideoContentGetResponse::Error {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
