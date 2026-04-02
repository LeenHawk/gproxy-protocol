//! Google Gemini API wire types.
//!
//! - [`generate_content`] — `POST models/{model}:generateContent` (request, response)
//! - [`stream_generate_content`] — streaming variant (NDJSON and SSE chunk types)
//! - [`count_tokens`] — `POST models/{model}:countTokens`
//! - [`embeddings`] / [`batch_embed_contents`] — embedding endpoints
//! - [`model_list`] / [`model_get`] — `GET models` catalog
//! - [`live`] — Live API WebSocket types (BidiGenerateContent)
//! - [`types`] — shared types: error types, response headers

pub mod types;

pub mod batch_embed_contents;
pub mod count_tokens;
pub mod embeddings;
pub mod generate_content;
pub mod live;
pub mod model_get;
pub mod model_list;
pub mod stream_generate_content;
