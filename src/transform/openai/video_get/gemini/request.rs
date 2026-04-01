use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::gemini::video_operation_get::request::{
    GeminiVideoOperationGetRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::video_get::request::OpenAiVideoGetRequest;
use crate::transform::openai::create_video::utils::decode_gemini_video_operation_id;

impl TryFrom<OpenAiVideoGetRequest> for GeminiVideoOperationGetRequest {
    type Error = crate::transform::utils::TransformError;

    fn try_from(value: OpenAiVideoGetRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            method: GeminiHttpMethod::Get,
            path: PathParameters {
                operation: decode_gemini_video_operation_id(&value.path.video_id)?,
            },
            query: QueryParameters::default(),
            headers: RequestHeaders {
                extra: value.headers.extra,
            },
            body: RequestBody::default(),
        })
    }
}
