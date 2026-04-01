use crate::claude::model_get::request::ClaudeModelGetRequest;
use crate::claude::model_get::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::claude::types::HttpMethod as ClaudeHttpMethod;
use crate::gemini::model_get::request::GeminiModelGetRequest;
use crate::transform::gemini::model_get::utils::strip_models_prefix;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelGetRequest> for ClaudeModelGetRequest {
    type Error = TransformError;

    fn try_from(value: GeminiModelGetRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: ClaudeHttpMethod::Get,
            path: PathParameters {
                model_id: strip_models_prefix(&value.path.name),
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
