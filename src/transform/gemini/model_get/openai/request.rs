use crate::gemini::model_get::request::GeminiModelGetRequest;
use crate::openai::model_get::request::OpenAiModelGetRequest;
use crate::openai::model_get::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::types::HttpMethod as OpenAiHttpMethod;
use crate::transform::gemini::model_get::utils::strip_models_prefix;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelGetRequest> for OpenAiModelGetRequest {
    type Error = TransformError;

    fn try_from(value: GeminiModelGetRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: OpenAiHttpMethod::Get,
            path: PathParameters {
                model: strip_models_prefix(&value.path.name),
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
