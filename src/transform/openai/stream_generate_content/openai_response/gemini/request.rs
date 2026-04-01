use crate::gemini::stream_generate_content::request::{
    GeminiStreamGenerateContentRequest,
    PathParameters as GeminiStreamGenerateContentPathParameters,
    QueryParameters as GeminiStreamGenerateContentQueryParameters,
    RequestHeaders as GeminiStreamGenerateContentRequestHeaders,
};
use crate::gemini::types::HttpMethod as GeminiHttpMethod;
use crate::openai::create_response::request::OpenAiCreateResponseRequest;
use crate::transform::utils::TransformError;

impl TryFrom<&OpenAiCreateResponseRequest> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &OpenAiCreateResponseRequest) -> Result<Self, TransformError> {
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
