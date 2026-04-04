//! Anthropic Claude Messages API wire types.
//!
//! - [`create_message`] — `POST /v1/messages` request, response, and stream events
//! - [`count_tokens`] — `POST /v1/messages/count_tokens` (also contains shared types used across operations)
//! - [`model_list`] / [`model_get`] — `GET /v1/models` catalog endpoints
//! - [`file_upload`] — `POST /v1/files` (multipart upload)
//! - [`file_list`] — `GET /v1/files` (paginated listing)
//! - [`file_download`] — `GET /v1/files/{file_id}/content` (raw binary download)
//! - [`file_get`] — `GET /v1/files/{file_id}` (metadata retrieval)
//! - [`file_delete`] — `DELETE /v1/files/{file_id}`
//! - [`types`] — shared types: `BetaModelInfo`, `FileMetadata`, error types, `AnthropicVersion`, beta headers

pub mod types;

pub mod count_tokens;
pub mod create_message;
pub mod file_delete;
pub mod file_download;
pub mod file_get;
pub mod file_list;
pub mod file_upload;
pub mod model_get;
pub mod model_list;
