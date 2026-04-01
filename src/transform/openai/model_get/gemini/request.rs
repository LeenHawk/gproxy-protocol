use crate::gemini::model_get::request::GeminiModelGetRequest;
use crate::gemini::model_get::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::openai::model_get::request::OpenAiModelGetRequest;
use crate::transform::gemini::model_get::utils::ensure_models_prefix;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiModelGetRequest> for GeminiModelGetRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiModelGetRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: GeminiHttpMethod::Get,
            path: PathParameters {
                name: ensure_models_prefix(&value.path.model),
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
