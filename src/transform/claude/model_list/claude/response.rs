use crate::claude::model_list::response::ClaudeModelListResponse;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeModelListResponse> for ClaudeModelListResponse {
    type Error = TransformError;

    fn try_from(value: &ClaudeModelListResponse) -> Result<Self, TransformError> {
        Ok(value.clone())
    }
}
