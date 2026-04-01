use crate::claude::create_message::request::ClaudeCreateMessageRequest;
use crate::openai::create_response::request::OpenAiCreateResponseRequest;
use crate::openai::create_response::types::HttpMethod as OpenAiHttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeCreateMessageRequest> for OpenAiCreateResponseRequest {
    type Error = TransformError;

    fn try_from(value: &ClaudeCreateMessageRequest) -> Result<Self, TransformError> {
        let mut output = OpenAiCreateResponseRequest::try_from(value.clone())?;
        output.method = OpenAiHttpMethod::Post;
        output.body.stream = Some(true);
        Ok(output)
    }
}
