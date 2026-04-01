use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;

/// A single NDJSON stream chunk (one line of newline-delimited JSON).
///
/// Each line in the NDJSON response deserializes to this type.
/// Used when `alt` query parameter is omitted (default transport).
pub type GeminiNdjsonChunk = GeminiGenerateContentResponseBody;

/// A single SSE stream chunk (the `data:` payload of one SSE event).
///
/// Each SSE `data:` line (except `[DONE]`) deserializes to this type.
/// Used when `alt=sse` query parameter is set.
pub type GeminiSseChunk = GeminiGenerateContentResponseBody;
