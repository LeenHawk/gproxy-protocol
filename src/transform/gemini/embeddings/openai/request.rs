use crate::gemini::embeddings::request::GeminiEmbedContentRequest;
use crate::openai::embeddings::request::{
    OpenAiEmbeddingsRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::embeddings::types::{HttpMethod, OpenAiEmbeddingInput, OpenAiEmbeddingModel};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiEmbedContentRequest> for OpenAiEmbeddingsRequest {
    type Error = TransformError;

    fn try_from(value: GeminiEmbedContentRequest) -> Result<Self, TransformError> {
        let mut input_texts = Vec::new();
        for part in value.body.content.parts {
            if let Some(text) = part.text
                && !text.is_empty()
            {
                input_texts.push(text);
            }
        }

        let input = match input_texts.len() {
            0 => OpenAiEmbeddingInput::String(String::new()),
            1 => OpenAiEmbeddingInput::String(input_texts.remove(0)),
            _ => OpenAiEmbeddingInput::StringArray(input_texts),
        };

        let raw_model = value
            .path
            .model
            .strip_prefix("models/")
            .unwrap_or(value.path.model.as_str())
            .to_string();

        let model = match raw_model.as_str() {
            "text-embedding-ada-002" => OpenAiEmbeddingModel::Known(
                crate::openai::embeddings::types::OpenAiEmbeddingModelKnown::TextEmbeddingAda002,
            ),
            "text-embedding-3-small" => OpenAiEmbeddingModel::Known(
                crate::openai::embeddings::types::OpenAiEmbeddingModelKnown::TextEmbedding3Small,
            ),
            "text-embedding-3-large" => OpenAiEmbeddingModel::Known(
                crate::openai::embeddings::types::OpenAiEmbeddingModelKnown::TextEmbedding3Large,
            ),
            _ => OpenAiEmbeddingModel::Custom(raw_model),
        };

        Ok(Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                input,
                model,
                dimensions: value.body.output_dimensionality,
                encoding_format: None,
                user: None,
            },
        })
    }
}
