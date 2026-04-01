use serde::{Deserialize, Serialize};

use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;

/// Parsed stream body for NDJSON transport (default when `alt` is omitted).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiNdjsonStreamBody {
    /// Incremental response chunks in receive order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chunks: Vec<GeminiGenerateContentResponseBody>,
}

/// Parsed stream body for SSE transport (`alt=sse`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiSseStreamBody {
    /// SSE events in receive order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<GeminiSseEvent>,
}

/// A single SSE event frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiSseEvent {
    /// Optional SSE `event` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// SSE `data` field payload.
    pub data: GeminiSseEventData,
}

/// SSE `data` payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GeminiSseEventData {
    /// A regular stream chunk with `GenerateContentResponse` shape.
    Chunk(GeminiGenerateContentResponseBody),
    /// Stream end marker (`[DONE]`).
    Done(String),
}

impl GeminiSseEventData {
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Done(marker) if marker == "[DONE]")
    }
}
