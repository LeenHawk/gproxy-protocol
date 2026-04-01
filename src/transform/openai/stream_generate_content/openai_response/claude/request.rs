use crate::claude::create_message::request::ClaudeCreateMessageRequest;
use crate::openai::create_response::request::OpenAiCreateResponseRequest;
use crate::transform::utils::TransformError;

impl TryFrom<&OpenAiCreateResponseRequest> for ClaudeCreateMessageRequest {
    type Error = TransformError;

    fn try_from(value: &OpenAiCreateResponseRequest) -> Result<Self, TransformError> {
        ClaudeCreateMessageRequest::try_from(value.clone())
    }
}
