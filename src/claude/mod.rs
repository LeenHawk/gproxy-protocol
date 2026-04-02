//! Anthropic Claude Messages API wire types.
//!
//! - [`create_message`] — `POST /v1/messages` request, response, and stream events
//! - [`count_tokens`] — `POST /v1/messages/count_tokens` (also contains shared types used across operations)
//! - [`model_list`] / [`model_get`] — `GET /v1/models` catalog endpoints
//! - [`types`] — shared types: `BetaModelInfo`, error types, `AnthropicVersion`, beta headers

pub mod types;

pub mod count_tokens;
pub mod create_message;
pub mod model_get;
pub mod model_list;
