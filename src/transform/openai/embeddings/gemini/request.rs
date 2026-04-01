use crate::gemini::count_tokens::types::GeminiPart;
use crate::gemini::embeddings::request::{
    GeminiEmbedContentRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::embeddings::types as gt;
use crate::openai::embeddings::request::OpenAiEmbeddingsRequest;
use crate::openai::embeddings::types as ot;
use crate::transform::gemini::model_get::utils::ensure_models_prefix;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiEmbeddingsRequest> for GeminiEmbedContentRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiEmbeddingsRequest) -> Result<Self, TransformError> {
        let input_parts = match value.body.input {
            ot::OpenAiEmbeddingInput::String(text) => vec![GeminiPart {
                text: Some(text),
                ..GeminiPart::default()
            }],
            ot::OpenAiEmbeddingInput::StringArray(texts) => {
                if texts.is_empty() {
                    vec![GeminiPart {
                        text: Some(String::new()),
                        ..GeminiPart::default()
                    }]
                } else {
                    texts
                        .into_iter()
                        .map(|text| GeminiPart {
                            text: Some(text),
                            ..GeminiPart::default()
                        })
                        .collect::<Vec<_>>()
                }
            }
            ot::OpenAiEmbeddingInput::TokenArray(tokens) => vec![GeminiPart {
                text: Some(
                    tokens
                        .into_iter()
                        .map(|token| token.to_string())
                        .collect::<Vec<_>>()
                        .join(" "),
                ),
                ..GeminiPart::default()
            }],
            ot::OpenAiEmbeddingInput::TokenArrayArray(token_batches) => {
                if token_batches.is_empty() {
                    vec![GeminiPart {
                        text: Some(String::new()),
                        ..GeminiPart::default()
                    }]
                } else {
                    token_batches
                        .into_iter()
                        .map(|tokens| GeminiPart {
                            text: Some(
                                tokens
                                    .into_iter()
                                    .map(|token| token.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" "),
                            ),
                            ..GeminiPart::default()
                        })
                        .collect::<Vec<_>>()
                }
            }
        };

        let model_name = match value.body.model {
            ot::OpenAiEmbeddingModel::Known(ot::OpenAiEmbeddingModelKnown::TextEmbeddingAda002) => {
                "text-embedding-ada-002".to_string()
            }
            ot::OpenAiEmbeddingModel::Known(ot::OpenAiEmbeddingModelKnown::TextEmbedding3Small) => {
                "text-embedding-3-small".to_string()
            }
            ot::OpenAiEmbeddingModel::Known(ot::OpenAiEmbeddingModelKnown::TextEmbedding3Large) => {
                "text-embedding-3-large".to_string()
            }
            ot::OpenAiEmbeddingModel::Custom(model) => model,
        };
        let model = ensure_models_prefix(&model_name);

        Ok(GeminiEmbedContentRequest {
            method: gt::HttpMethod::Post,
            path: PathParameters { model },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                content: gt::GeminiContent {
                    parts: input_parts,
                    role: None,
                },
                task_type: None,
                title: None,
                output_dimensionality: value.body.dimensions,
            },
        })
    }
}
