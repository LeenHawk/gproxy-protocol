use crate::claude::create_message::request::ClaudeCreateMessageRequest;
use crate::openai::create_chat_completions::request::OpenAiChatCompletionsRequest;
use crate::openai::create_chat_completions::types::HttpMethod as OpenAiHttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeCreateMessageRequest> for OpenAiChatCompletionsRequest {
    type Error = TransformError;

    fn try_from(value: &ClaudeCreateMessageRequest) -> Result<Self, TransformError> {
        let mut output = OpenAiChatCompletionsRequest::try_from(value.clone())?;
        output.method = OpenAiHttpMethod::Post;
        output.body.stream = Some(true);
        Ok(output)
    }
}
