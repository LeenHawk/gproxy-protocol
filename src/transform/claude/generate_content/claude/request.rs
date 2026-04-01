use crate::claude::create_message::request::ClaudeCreateMessageRequest;
use crate::claude::create_message::types::HttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeCreateMessageRequest> for ClaudeCreateMessageRequest {
    type Error = TransformError;

    fn try_from(value: &ClaudeCreateMessageRequest) -> Result<Self, TransformError> {
        let mut output = value.clone();
        output.method = HttpMethod::Post;
        Ok(output)
    }
}
