use crate::claude::model_list::request::ClaudeModelListRequest;
use crate::claude::model_list::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::claude::types::HttpMethod as ClaudeHttpMethod;
use crate::gemini::model_list::request::GeminiModelListRequest;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiModelListRequest> for ClaudeModelListRequest {
    type Error = TransformError;

    fn try_from(value: GeminiModelListRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: ClaudeHttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters {
                after_id: value.query.page_token,
                before_id: None,
                limit: value.query.page_size.and_then(|v| u16::try_from(v).ok()),
            },
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
