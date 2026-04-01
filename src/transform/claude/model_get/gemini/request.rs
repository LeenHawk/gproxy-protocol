use crate::claude::model_get::request::ClaudeModelGetRequest;
use crate::gemini::model_get::request::{
    GeminiModelGetRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::transform::claude::model_list::gemini::utils::ensure_models_prefix;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelGetRequest> for GeminiModelGetRequest {
    type Error = TransformError;

    fn try_from(value: ClaudeModelGetRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: GeminiHttpMethod::Get,
            path: PathParameters {
                name: ensure_models_prefix(&value.path.model_id),
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
