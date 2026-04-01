use crate::gemini::generate_content::request::{
    GeminiGenerateContentRequest, PathParameters as GeminiGenerateContentPathParameters,
    QueryParameters as GeminiGenerateContentQueryParameters,
    RequestHeaders as GeminiGenerateContentRequestHeaders,
};
use crate::gemini::generate_content::types::HttpMethod as GeminiGenerateContentHttpMethod;
use crate::gemini::stream_generate_content::request::GeminiStreamGenerateContentRequest;
use crate::openai::create_chat_completions::request::OpenAiChatCompletionsRequest;
use crate::openai::create_chat_completions::types::HttpMethod as OpenAiHttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiStreamGenerateContentRequest> for OpenAiChatCompletionsRequest {
    type Error = TransformError;

    fn try_from(value: GeminiStreamGenerateContentRequest) -> Result<Self, TransformError> {
        let mut output = OpenAiChatCompletionsRequest::try_from(GeminiGenerateContentRequest {
            method: GeminiGenerateContentHttpMethod::Post,
            path: GeminiGenerateContentPathParameters {
                model: value.path.model,
            },
            query: GeminiGenerateContentQueryParameters::default(),
            headers: GeminiGenerateContentRequestHeaders::default(),
            body: value.body,
        })?;
        output.method = OpenAiHttpMethod::Post;
        output.body.stream = Some(true);
        Ok(output)
    }
}
