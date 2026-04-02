//! OpenAI API wire types.
//!
//! - [`create_chat_completions`] — `POST /v1/chat/completions` (request, response, stream chunks)
//! - [`create_response`] — `POST /v1/responses` (request, response, stream events, WebSocket frames)
//! - [`compact_response`] — `POST /v1/responses/{id}/compact`
//! - [`count_tokens`] — `POST /v1/responses/input_tokens/count`
//! - [`embeddings`] — `POST /v1/embeddings`
//! - [`create_image`] / [`create_image_edit`] — image generation and editing
//! - [`model_list`] / [`model_get`] — `GET /v1/models` catalog
//! - [`types`] — shared types: error types, model enums, common structures

pub mod types;

pub mod compact_response;
pub mod count_tokens;
pub mod create_chat_completions;
pub mod create_image;
pub mod create_image_edit;
pub mod create_response;
pub mod embeddings;
pub mod model_get;
pub mod model_list;
