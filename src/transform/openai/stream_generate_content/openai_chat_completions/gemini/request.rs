use crate::gemini::stream_generate_content::request::{
    GeminiStreamGenerateContentRequest,
    PathParameters as GeminiStreamGenerateContentPathParameters,
    QueryParameters as GeminiStreamGenerateContentQueryParameters,
    RequestHeaders as GeminiStreamGenerateContentRequestHeaders,
};
use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::openai::create_chat_completions::request::OpenAiChatCompletionsRequest;
use crate::transform::utils::TransformError;

impl TryFrom<&OpenAiChatCompletionsRequest> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &OpenAiChatCompletionsRequest) -> Result<Self, TransformError> {
        let output =
            crate::gemini::generate_content::request::GeminiGenerateContentRequest::try_from(
                value.clone(),
            )?;

        Ok(Self {
            method: GeminiHttpMethod::Post,
            path: GeminiStreamGenerateContentPathParameters {
                model: output.path.model,
            },
            query: GeminiStreamGenerateContentQueryParameters::default(),
            headers: GeminiStreamGenerateContentRequestHeaders::default(),
            body: output.body,
        })
    }
}
