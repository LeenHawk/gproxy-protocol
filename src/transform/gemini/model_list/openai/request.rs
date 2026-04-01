use crate::gemini::model_list::request::GeminiModelListRequest;
use crate::openai::model_list::request::OpenAiModelListRequest;
use crate::openai::model_list::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::types::HttpMethod as OpenAiHttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelListRequest> for OpenAiModelListRequest {
    type Error = TransformError;

    fn try_from(_value: GeminiModelListRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: OpenAiHttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
