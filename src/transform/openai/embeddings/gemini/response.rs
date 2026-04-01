use crate::gemini::embeddings::response::GeminiEmbedContentResponse;
use crate::openai::embeddings::response::OpenAiEmbeddingsResponse;
use crate::openai::embeddings::types::{
    OpenAiCreateEmbeddingResponse, OpenAiEmbeddingData, OpenAiEmbeddingDataObject,
    OpenAiEmbeddingResponseObject, OpenAiEmbeddingUsage, OpenAiEmbeddingVector,
};
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::openai::model_list::gemini::utils::{
    openai_error_response_from_gemini, strip_models_prefix,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiEmbedContentResponse> for OpenAiEmbeddingsResponse {
    type Error = TransformError;

    fn try_from(value: GeminiEmbedContentResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiEmbedContentResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let model = headers
                    .extra
                    .get("x-goog-request-params")
                    .or_else(|| headers.extra.get("X-Goog-Request-Params"))
                    .and_then(|params| {
                        params.split('&').find_map(|pair| {
                            let (key, value) = pair.split_once('=')?;
                            if key == "model" {
                                Some(strip_models_prefix(value))
                            } else {
                                None
                            }
                        })
                    })
                    .unwrap_or_default();

                OpenAiEmbeddingsResponse::Success {
                    stats_code,
                    headers: OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: OpenAiCreateEmbeddingResponse {
                        data: vec![OpenAiEmbeddingData {
                            embedding: OpenAiEmbeddingVector::FloatArray(body.embedding.values),
                            index: 0,
                            object: OpenAiEmbeddingDataObject::Embedding,
                        }],
                        model,
                        object: OpenAiEmbeddingResponseObject::List,
                        usage: OpenAiEmbeddingUsage::default(),
                    },
                }
            }
            GeminiEmbedContentResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiEmbeddingsResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
