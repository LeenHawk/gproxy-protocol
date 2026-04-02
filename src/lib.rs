//! # gproxy-protocol
//!
//! Wire format types and cross-protocol transforms for **Claude**, **OpenAI**, and **Gemini** LLM APIs.
//!
//! This crate provides two layers:
//!
//! 1. **Protocol types** — Serde-compatible request, response, and stream payload structs for each API.
//! 2. **Transforms** — Cross-protocol converters that translate requests and responses between any pair of protocols.
//!
//! ## Protocol Modules
//!
//! | Module | API | Key Types |
//! |--------|-----|-----------|
//! | [`claude`] | Anthropic Messages API | `CreateMessageRequest`, `BetaMessage`, `ClaudeStreamEvent` |
//! | [`openai`] | OpenAI Chat Completions & Responses API | `ChatCompletionRequest`, `ResponseBody`, `ChatCompletionChunk`, `ResponseStreamEvent` |
//! | [`gemini`] | Google Gemini GenerateContent API | `GeminiGenerateContentRequest`, `GeminiGenerateContentResponse` |
//!
//! Each module follows the same internal structure:
//! - `types.rs` — shared types and enums
//! - `{operation}/request.rs` — request structs
//! - `{operation}/response.rs` — response structs
//! - `{operation}/stream.rs` — stream event payload types (no SSE/NDJSON framing)
//!
//! ## Deserializing a Request
//!
//! ```rust
//! use gproxy_protocol::openai::create_chat_completions::request::RequestBody;
//!
//! let json = r#"{"model": "gpt-4", "messages": [{"role": "user", "content": "Hello"}]}"#;
//! let body: RequestBody = serde_json::from_str(json).unwrap();
//! assert_eq!(body.model.as_deref(), Some("gpt-4"));
//! ```
//!
//! ## Cross-Protocol Transforms
//!
//! The [`transform`] module converts between protocols using Rust's `TryFrom` trait.
//! Transforms are organized as `transform::{source_protocol}::{operation}::{target_protocol}`.
//!
//! ### Non-streaming request conversion
//!
//! ```rust
//! use gproxy_protocol::claude::create_message::request::ClaudeCreateMessageRequest;
//! use gproxy_protocol::openai::create_chat_completions::request::OpenAiChatCompletionsRequest;
//!
//! // Claude request → OpenAI ChatCompletions request
//! fn convert(claude_req: ClaudeCreateMessageRequest) -> Result<OpenAiChatCompletionsRequest, gproxy_protocol::transform::TransformError> {
//!     OpenAiChatCompletionsRequest::try_from(claude_req)
//! }
//! ```
//!
//! ### Streaming response conversion (buffer reuse pattern)
//!
//! Stream converters process chunks one at a time. The caller provides a reusable
//! `Vec` buffer to avoid per-chunk heap allocation:
//!
//! ```rust,ignore
//! use gproxy_protocol::openai::create_chat_completions::stream::ChatCompletionChunk;
//! use gproxy_protocol::claude::create_message::stream::ClaudeStreamEvent;
//! use gproxy_protocol::transform::claude::stream_generate_content::openai_chat_completions::response::OpenAiChatCompletionsToClaudeStream;
//!
//! let mut converter = OpenAiChatCompletionsToClaudeStream::default();
//! let mut buf: Vec<ClaudeStreamEvent> = Vec::new();
//!
//! // For each incoming chunk from upstream:
//! // converter.on_chunk(chunk, &mut buf);
//! // buf now contains 0..N output events — forward them, then clear:
//! // for event in buf.drain(..) { send_downstream(event); }
//!
//! // When the upstream stream ends:
//! // converter.finish(&mut buf);
//! // forward remaining events in buf
//! ```
//!
//! ## SSE-to-NDJSON Rewriter
//!
//! The [`stream`] module provides an incremental SSE-to-NDJSON byte converter,
//! useful when the downstream expects NDJSON but the internal pipeline uses SSE framing:
//!
//! ```rust
//! use gproxy_protocol::stream::SseToNdjsonRewriter;
//!
//! let mut rewriter = SseToNdjsonRewriter::default();
//! let out = rewriter.push_chunk(b"data: {\"text\":\"hi\"}\n\n");
//! assert_eq!(out, b"{\"text\":\"hi\"}\n");
//! ```

pub mod claude;
pub mod gemini;
pub mod openai;

pub mod stream;
pub mod transform;
