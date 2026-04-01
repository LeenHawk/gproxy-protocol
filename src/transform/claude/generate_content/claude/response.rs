use crate::claude::create_message::response::ClaudeCreateMessageResponse;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeCreateMessageResponse> for ClaudeCreateMessageResponse {
    type Error = TransformError;

    fn try_from(value: &ClaudeCreateMessageResponse) -> Result<Self, TransformError> {
        Ok(value.clone())
    }
}
