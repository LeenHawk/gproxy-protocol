use crate::gemini::generate_videos::response::GeminiGenerateVideosResponse;
use crate::openai::create_video::response::OpenAiCreateVideoResponse;
use crate::transform::openai::create_image::gemini::utils::openai_response_headers_from_gemini;
use crate::transform::openai::create_video::utils::openai_video_from_gemini_operation;
use crate::transform::openai::model_list::gemini::utils::openai_error_response_from_gemini;

impl TryFrom<GeminiGenerateVideosResponse> for OpenAiCreateVideoResponse {
    type Error = crate::transform::utils::TransformError;

    fn try_from(
        value: GeminiGenerateVideosResponse,
    ) -> Result<Self, crate::transform::utils::TransformError> {
        Ok(match value {
            GeminiGenerateVideosResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiCreateVideoResponse::Success {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: openai_video_from_gemini_operation(*body),
            },
            GeminiGenerateVideosResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCreateVideoResponse::Error {
                stats_code,
                headers: openai_response_headers_from_gemini(headers),
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
