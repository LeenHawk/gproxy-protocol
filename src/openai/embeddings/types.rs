use serde::{Deserialize, Serialize};

pub use crate::openai::types::{
    HttpMethod, OpenAiApiError, OpenAiApiErrorResponse, OpenAiResponseHeaders,
};

/// Input union accepted by OpenAI `/embeddings`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiEmbeddingInput {
    String(String),
    StringArray(Vec<String>),
    TokenArray(Vec<i64>),
    TokenArrayArray(Vec<Vec<i64>>),
}

/// Supported embedding model names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiEmbeddingModel {
    Known(OpenAiEmbeddingModelKnown),
    Custom(String),
}

/// Known embedding model constants from upstream docs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiEmbeddingModelKnown {
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
}

/// Encoding format for returned embedding values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiEmbeddingEncodingFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "base64")]
    Base64,
}

/// A single embedding object in the response list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiEmbeddingData {
    /// Embedding vector, encoded as float list or base64 string.
    pub embedding: OpenAiEmbeddingVector,
    /// Position of this embedding in the request input list.
    pub index: u64,
    /// Object discriminator, always `embedding`.
    pub object: OpenAiEmbeddingDataObject,
}

/// Embedding payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiEmbeddingVector {
    FloatArray(Vec<f64>),
    Base64(String),
}

/// OpenAI embedding item object discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiEmbeddingDataObject {
    #[serde(rename = "embedding")]
    Embedding,
}

/// Usage metrics for embeddings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiEmbeddingUsage {
    pub prompt_tokens: u64,
    pub total_tokens: u64,
}

/// Successful response payload for OpenAI `/embeddings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateEmbeddingResponse {
    pub data: Vec<OpenAiEmbeddingData>,
    pub model: String,
    pub object: OpenAiEmbeddingResponseObject,
    pub usage: OpenAiEmbeddingUsage,
}

/// OpenAI embeddings response object discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiEmbeddingResponseObject {
    #[serde(rename = "list")]
    List,
}
