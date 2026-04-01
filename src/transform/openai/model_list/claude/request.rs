use crate::claude::model_list::request::ClaudeModelListRequest;
use crate::claude::model_list::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::claude::types::HttpMethod as ClaudeHttpMethod;
use crate::openai::model_list::request::OpenAiModelListRequest;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelListRequest> for ClaudeModelListRequest {
    type Error = TransformError;

    fn try_from(_value: OpenAiModelListRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: ClaudeHttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
