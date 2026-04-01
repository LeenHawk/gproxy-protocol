use crate::gemini::stream_generate_content::stream::GeminiSseChunk;
use crate::gemini::types::JsonObject;

pub fn parse_json_object_or_empty(input: &str) -> JsonObject {
    serde_json::from_str::<JsonObject>(input).unwrap_or_default()
}

/// Placeholder "done" sentinel — callers that need to signal stream end
/// can use `is_finished()` on the converter instead.  This is kept only
/// for the `nonstream_to_stream` path that still needs to materialise a
/// final chunk, which is simply an empty body.
pub fn empty_chunk() -> GeminiSseChunk {
    GeminiSseChunk::default()
}
