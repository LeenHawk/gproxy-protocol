use crate::claude::model_get::request::ClaudeModelGetRequest;
use crate::openai::model_get::request::{
    OpenAiModelGetRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::types::HttpMethod as OpenAiHttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelGetRequest> for OpenAiModelGetRequest {
    type Error = TransformError;

    fn try_from(value: ClaudeModelGetRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: OpenAiHttpMethod::Get,
            path: PathParameters {
                model: value.path.model_id,
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
