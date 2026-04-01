use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::gemini::video_content_get::request::{
    GeminiVideoContentGetRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::video_content_get::types::GeminiVideoContentVariant;
use crate::openai::video_content_get::request::OpenAiVideoContentGetRequest;
use crate::openai::video_content_get::types::OpenAiVideoContentVariant;
use crate::transform::openai::create_video::utils::decode_gemini_video_operation_id;
use crate::transform::utils::TransformError;

fn gemini_variant(
    value: Option<OpenAiVideoContentVariant>,
) -> Result<Option<GeminiVideoContentVariant>, TransformError> {
    match value {
        None | Some(OpenAiVideoContentVariant::Video) => Ok(Some(GeminiVideoContentVariant::Video)),
        Some(OpenAiVideoContentVariant::Thumbnail)
        | Some(OpenAiVideoContentVariant::Spritesheet) => Err(TransformError::not_implemented(
            "cannot convert OpenAI video content request with non-video variant to Gemini Veo request",
        )),
    }
}

impl TryFrom<OpenAiVideoContentGetRequest> for GeminiVideoContentGetRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiVideoContentGetRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            method: GeminiHttpMethod::Get,
            path: PathParameters {
                operation: decode_gemini_video_operation_id(&value.path.video_id)?,
            },
            query: QueryParameters {
                variant: gemini_variant(value.query.variant)?,
            },
            headers: RequestHeaders {
                extra: value.headers.extra,
            },
            body: RequestBody::default(),
        })
    }
}
