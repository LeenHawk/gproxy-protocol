use crate::gemini::model_list::request::GeminiModelListRequest;
use crate::gemini::model_list::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::openai::model_list::request::OpenAiModelListRequest;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelListRequest> for GeminiModelListRequest {
    type Error = TransformError;

    fn try_from(_value: OpenAiModelListRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: GeminiHttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
