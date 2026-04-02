use std::collections::BTreeMap;

use crate::claude::create_message::stream::{BetaRawContentBlockDelta, ClaudeStreamEvent};
use crate::claude::create_message::types::{BetaContentBlock, BetaStopReason};
use crate::claude::types::BetaError;
use crate::gemini::count_tokens::types::{GeminiContentRole, GeminiFunctionCall, GeminiPart};
use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;
use crate::gemini::generate_content::types::{
    GeminiBlockReason, GeminiCandidate, GeminiContent, GeminiFinishReason, GeminiPromptFeedback,
    GeminiUsageMetadata,
};
use crate::transform::claude::utils::claude_model_to_string;
use crate::transform::gemini::stream_generate_content::utils::parse_json_object_or_empty;
use crate::transform::utils::TransformError;

#[derive(Debug, Clone)]
enum ClaudeBlockState {
    Thinking {
        signature: String,
    },
    ToolUse {
        id: String,
        name: String,
        partial_json: String,
    },
    Other,
}

#[derive(Debug, Default, Clone)]
pub struct ClaudeToGeminiStream {
    response_id: Option<String>,
    model_version: Option<String>,
    input_tokens: u64,
    cache_creation_input_tokens: u64,
    cached_input_tokens: u64,
    output_tokens: u64,
    usage_metadata: Option<GeminiUsageMetadata>,
    blocks: BTreeMap<u64, ClaudeBlockState>,
    finished: bool,
}

impl ClaudeToGeminiStream {
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn usage_from_counts(
        input_tokens: u64,
        cache_creation_tokens: u64,
        cached_tokens: u64,
        output_tokens: u64,
    ) -> GeminiUsageMetadata {
        let prompt_tokens = input_tokens.saturating_add(cache_creation_tokens);
        GeminiUsageMetadata {
            prompt_token_count: Some(prompt_tokens),
            cached_content_token_count: Some(cached_tokens),
            candidates_token_count: Some(output_tokens),
            total_token_count: Some(
                prompt_tokens
                    .saturating_add(cached_tokens)
                    .saturating_add(output_tokens),
            ),
            ..GeminiUsageMetadata::default()
        }
    }

    fn sync_usage_metadata(&mut self) {
        self.usage_metadata = Some(Self::usage_from_counts(
            self.input_tokens,
            self.cache_creation_input_tokens,
            self.cached_input_tokens,
            self.output_tokens,
        ));
    }

    fn finish_reason_from_stop_reason(stop_reason: Option<BetaStopReason>) -> GeminiFinishReason {
        match stop_reason {
            Some(BetaStopReason::MaxTokens) | Some(BetaStopReason::ModelContextWindowExceeded) => {
                GeminiFinishReason::MaxTokens
            }
            Some(BetaStopReason::ToolUse) => GeminiFinishReason::UnexpectedToolCall,
            Some(BetaStopReason::Refusal) => GeminiFinishReason::Safety,
            Some(BetaStopReason::Compaction) | Some(BetaStopReason::PauseTurn) => {
                GeminiFinishReason::Other
            }
            Some(BetaStopReason::EndTurn) | Some(BetaStopReason::StopSequence) | None => {
                GeminiFinishReason::Stop
            }
        }
    }

    fn error_message(error: BetaError) -> String {
        match error {
            BetaError::InvalidRequest(error) => error.message,
            BetaError::Authentication(error) => error.message,
            BetaError::Billing(error) => error.message,
            BetaError::Permission(error) => error.message,
            BetaError::NotFound(error) => error.message,
            BetaError::RateLimit(error) => error.message,
            BetaError::GatewayTimeout(error) => error.message,
            BetaError::Api(error) => error.message,
            BetaError::Overloaded(error) => error.message,
        }
    }

    fn chunk_from_parts(
        &self,
        parts: Vec<GeminiPart>,
        finish_reason: Option<GeminiFinishReason>,
        prompt_feedback: Option<GeminiPromptFeedback>,
    ) -> GeminiGenerateContentResponseBody {
        GeminiGenerateContentResponseBody {
            candidates: Some(vec![GeminiCandidate {
                content: Some(GeminiContent {
                    parts,
                    role: Some(GeminiContentRole::Model),
                }),
                finish_reason,
                index: Some(0),
                ..GeminiCandidate::default()
            }]),
            prompt_feedback,
            usage_metadata: self.usage_metadata.clone(),
            model_version: self.model_version.clone(),
            response_id: self.response_id.clone(),
            model_status: None,
        }
    }

    fn text_chunk(&self, text: String) -> Option<GeminiGenerateContentResponseBody> {
        if text.is_empty() {
            None
        } else {
            Some(self.chunk_from_parts(
                vec![GeminiPart {
                    text: Some(text),
                    ..GeminiPart::default()
                }],
                None,
                None,
            ))
        }
    }

    fn thinking_chunk(
        &self,
        signature: String,
        thinking: String,
    ) -> Option<GeminiGenerateContentResponseBody> {
        if thinking.is_empty() {
            None
        } else {
            Some(self.chunk_from_parts(
                vec![GeminiPart {
                    thought: Some(true),
                    thought_signature: Some(signature),
                    text: Some(thinking),
                    ..GeminiPart::default()
                }],
                None,
                None,
            ))
        }
    }

    fn function_call_chunk(
        &self,
        id: String,
        name: String,
        arguments: String,
    ) -> GeminiGenerateContentResponseBody {
        self.chunk_from_parts(
            vec![GeminiPart {
                function_call: Some(GeminiFunctionCall {
                    id: Some(id),
                    name,
                    args: Some(parse_json_object_or_empty(&arguments)),
                }),
                ..GeminiPart::default()
            }],
            None,
            None,
        )
    }

    pub fn on_event(
        &mut self,
        event: ClaudeStreamEvent,
        out: &mut Vec<GeminiGenerateContentResponseBody>,
    ) -> Result<(), TransformError> {
        if self.finished {
            return Ok(());
        }

        match event {
            ClaudeStreamEvent::MessageStart { message } => {
                self.response_id = Some(message.id);
                self.model_version = Some(claude_model_to_string(&message.model));
                self.input_tokens = message.usage.input_tokens;
                self.cache_creation_input_tokens = message.usage.cache_creation_input_tokens;
                self.cached_input_tokens = message.usage.cache_read_input_tokens;
                self.output_tokens = message.usage.output_tokens;
                self.sync_usage_metadata();
            }
            ClaudeStreamEvent::ContentBlockStart {
                content_block,
                index,
            } => {
                let state = match content_block {
                    BetaContentBlock::Thinking(block) => ClaudeBlockState::Thinking {
                        signature: block.signature,
                    },
                    BetaContentBlock::ToolUse(block) => ClaudeBlockState::ToolUse {
                        id: block.id,
                        name: block.name,
                        partial_json: String::new(),
                    },
                    _ => ClaudeBlockState::Other,
                };
                self.blocks.insert(index, state);
            }
            ClaudeStreamEvent::ContentBlockDelta { delta, index } => match delta {
                BetaRawContentBlockDelta::Text { text } => {
                    if let Some(chunk) = self.text_chunk(text) {
                        out.push(chunk);
                    }
                }
                BetaRawContentBlockDelta::Thinking { thinking } => {
                    let signature = match self.blocks.get(&index) {
                        Some(ClaudeBlockState::Thinking { signature }) => signature.clone(),
                        _ => format!("thought_{index}"),
                    };
                    if let Some(chunk) = self.thinking_chunk(signature, thinking) {
                        out.push(chunk);
                    }
                }
                BetaRawContentBlockDelta::InputJson { partial_json } => {
                    let mut tool_snapshot = None;
                    if let Some(ClaudeBlockState::ToolUse {
                        id,
                        name,
                        partial_json: accumulated,
                    }) = self.blocks.get_mut(&index)
                    {
                        accumulated.push_str(&partial_json);
                        tool_snapshot = Some((id.clone(), name.clone(), accumulated.clone()));
                    }
                    if let Some((id, name, arguments)) = tool_snapshot {
                        out.push(self.function_call_chunk(id, name, arguments));
                    }
                }
                BetaRawContentBlockDelta::Signature { signature } => {
                    if let Some(ClaudeBlockState::Thinking { signature: sig }) =
                        self.blocks.get_mut(&index)
                    {
                        *sig = signature;
                    }
                }
                BetaRawContentBlockDelta::Compaction { content } => {
                    if let Some(content) = content
                        && let Some(chunk) = self.text_chunk(content)
                    {
                        out.push(chunk);
                    }
                }
                BetaRawContentBlockDelta::Citations { .. } => {}
            },
            ClaudeStreamEvent::ContentBlockStop { index } => {
                self.blocks.remove(&index);
            }
            ClaudeStreamEvent::MessageDelta {
                delta,
                usage,
                context_management: _,
            } => {
                if let Some(input_tokens) = usage.input_tokens {
                    self.input_tokens = input_tokens;
                }
                if let Some(cache_creation_input_tokens) = usage.cache_creation_input_tokens {
                    self.cache_creation_input_tokens = cache_creation_input_tokens;
                }
                if let Some(cached_input_tokens) = usage.cache_read_input_tokens {
                    self.cached_input_tokens = cached_input_tokens;
                }
                self.output_tokens = usage.output_tokens;
                self.sync_usage_metadata();

                let finish_reason = Self::finish_reason_from_stop_reason(delta.stop_reason);
                let prompt_feedback = if matches!(finish_reason, GeminiFinishReason::Safety) {
                    Some(GeminiPromptFeedback {
                        block_reason: Some(GeminiBlockReason::Safety),
                        safety_ratings: None,
                    })
                } else {
                    None
                };

                out.push(self.chunk_from_parts(Vec::new(), Some(finish_reason), prompt_feedback));
            }
            ClaudeStreamEvent::MessageStop {} => {
                self.finished = true;
            }
            ClaudeStreamEvent::Error { error } => {
                let message = Self::error_message(error);
                if let Some(chunk) = self.text_chunk(message) {
                    out.push(chunk);
                }
                self.finished = true;
            }
            ClaudeStreamEvent::Ping {} => {}
        }

        Ok(())
    }
}
