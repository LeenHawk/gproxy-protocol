use serde::{Deserialize, Serialize};

pub use crate::gemini::count_tokens::types::{GeminiContent, HttpMethod};
pub use crate::gemini::types::{GeminiApiError, GeminiApiErrorResponse, GeminiResponseHeaders};

/// A list of floats representing one embedding vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiContentEmbedding {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<f64>,
}

/// Type of task for which embeddings will be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiTaskType {
    #[serde(rename = "TASK_TYPE_UNSPECIFIED")]
    TaskTypeUnspecified,
    #[serde(rename = "RETRIEVAL_QUERY")]
    RetrievalQuery,
    #[serde(rename = "RETRIEVAL_DOCUMENT")]
    RetrievalDocument,
    #[serde(rename = "SEMANTIC_SIMILARITY")]
    SemanticSimilarity,
    #[serde(rename = "CLASSIFICATION")]
    Classification,
    #[serde(rename = "CLUSTERING")]
    Clustering,
    #[serde(rename = "QUESTION_ANSWERING")]
    QuestionAnswering,
    #[serde(rename = "FACT_VERIFICATION")]
    FactVerification,
    #[serde(rename = "CODE_RETRIEVAL_QUERY")]
    CodeRetrievalQuery,
}
