use crate::claude::model_list::request::ClaudeModelListRequest;
use crate::gemini::model_list::request::{
    GeminiModelListRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeModelListRequest> for GeminiModelListRequest {
    type Error = TransformError;

    fn try_from(value: ClaudeModelListRequest) -> Result<Self, TransformError> {
        Ok(Self {
            method: GeminiHttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters {
                page_size: value.query.limit.map(u32::from),
                page_token: value.query.after_id.or(value.query.before_id),
            },
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        })
    }
}
