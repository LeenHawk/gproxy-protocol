use crate::claude::create_message::request::ClaudeCreateMessageRequest;
use crate::openai::create_chat_completions::request::OpenAiChatCompletionsRequest;
use crate::transform::utils::TransformError;

impl TryFrom<&OpenAiChatCompletionsRequest> for ClaudeCreateMessageRequest {
    type Error = TransformError;

    fn try_from(value: &OpenAiChatCompletionsRequest) -> Result<Self, TransformError> {
        ClaudeCreateMessageRequest::try_from(value.clone())
    }
}
