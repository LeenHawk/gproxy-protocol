use crate::claude::model_list::request::ClaudeModelListRequest;
use crate::openai::model_list::request::{
    OpenAiModelListRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::types::HttpMethod as OpenAiHttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelListRequest> for OpenAiModelListRequest {
    type Error = TransformError;

    fn try_from(_value: ClaudeModelListRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: OpenAiHttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
