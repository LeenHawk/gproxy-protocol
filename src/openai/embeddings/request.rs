use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::embeddings::types::{
    HttpMethod, OpenAiEmbeddingEncodingFormat, OpenAiEmbeddingInput, OpenAiEmbeddingModel,
};

/// Request descriptor for OpenAI `embeddings.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiEmbeddingsRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body.
    pub body: RequestBody,
}

impl Default for OpenAiEmbeddingsRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        }
    }
}

/// OpenAI `/embeddings` does not define path params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// OpenAI `/embeddings` does not define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Request body for OpenAI `/embeddings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestBody {
    /// Input text or tokens to embed.
    pub input: OpenAiEmbeddingInput,
    /// Target embedding model.
    pub model: OpenAiEmbeddingModel,
    /// Optional output vector dimensionality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    /// Optional output encoding format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<OpenAiEmbeddingEncodingFormat>,
    /// Optional caller-specified end-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for RequestBody {
    fn default() -> Self {
        Self {
            input: OpenAiEmbeddingInput::String(String::new()),
            model: OpenAiEmbeddingModel::Custom(String::new()),
            dimensions: None,
            encoding_format: None,
            user: None,
        }
    }
}
