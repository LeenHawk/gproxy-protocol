use crate::claude::model_get::response::ClaudeModelGetResponse;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeModelGetResponse> for ClaudeModelGetResponse {
    type Error = TransformError;

    fn try_from(value: &ClaudeModelGetResponse) -> Result<Self, TransformError> {
        Ok(value.clone())
    }
}
