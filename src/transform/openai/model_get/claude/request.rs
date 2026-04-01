use crate::claude::model_get::request::ClaudeModelGetRequest;
use crate::claude::model_get::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::claude::types::HttpMethod as ClaudeHttpMethod;
use crate::openai::model_get::request::OpenAiModelGetRequest;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelGetRequest> for ClaudeModelGetRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiModelGetRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: ClaudeHttpMethod::Get,
            path: PathParameters {
                model_id: value.path.model,
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
