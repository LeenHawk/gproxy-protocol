use std::collections::BTreeMap;

use crate::claude::create_message::stream::ClaudeStreamEvent;
use crate::claude::create_message::types::{BetaServiceTier, BetaStopReason};
use crate::openai::create_chat_completions::stream::ChatCompletionChunk;
use crate::openai::create_chat_completions::types::{
    ChatCompletionFinishReason, ChatCompletionServiceTier, CompletionUsage,
};
use crate::transform::claude::stream_generate_content::utils::{
    input_json_delta_event, message_delta_event, message_start_event, message_stop_event,
    push_text_block, start_tool_use_block_event, stop_block_event,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamState {
    Init,
    Running,
    Finished,
}

#[derive(Debug, Clone)]
pub struct OpenAiChatCompletionsToClaudeStream {
    state: StreamState,
    next_block_index: u64,
    open_tool_blocks: BTreeMap<u32, u64>,
    open_legacy_function_block: Option<u64>,
    message_id: String,
    model: String,
    service_tier: BetaServiceTier,
    input_tokens: u64,
    cached_input_tokens: u64,
    output_tokens: u64,
    stop_reason: Option<BetaStopReason>,
    has_tool_use: bool,
    has_refusal: bool,
}

impl Default for OpenAiChatCompletionsToClaudeStream {
    fn default() -> Self {
        Self {
            state: StreamState::Init,
            next_block_index: 0,
            open_tool_blocks: BTreeMap::new(),
            open_legacy_function_block: None,
            message_id: String::new(),
            model: String::new(),
            service_tier: BetaServiceTier::Standard,
            input_tokens: 0,
            cached_input_tokens: 0,
            output_tokens: 0,
            stop_reason: None,
            has_tool_use: false,
            has_refusal: false,
        }
    }
}

impl OpenAiChatCompletionsToClaudeStream {
    pub fn is_finished(&self) -> bool {
        matches!(self.state, StreamState::Finished)
    }

    fn apply_usage(&mut self, usage: &CompletionUsage) {
        let cached_tokens = usage
            .prompt_tokens_details
            .as_ref()
            .and_then(|details| details.cached_tokens)
            .unwrap_or(0);
        let total_input_tokens = if usage.total_tokens >= usage.completion_tokens {
            usage.total_tokens.saturating_sub(usage.completion_tokens)
        } else {
            usage.prompt_tokens
        };
        self.input_tokens = total_input_tokens.saturating_sub(cached_tokens);
        self.cached_input_tokens = cached_tokens;
        self.output_tokens = usage.completion_tokens;
    }

    pub fn on_chunk(&mut self, chunk: ChatCompletionChunk) -> Vec<ClaudeStreamEvent> {
        if self.is_finished() {
            return Vec::new();
        }

        let mut out = Vec::new();
        let chunk_service_tier = chunk.service_tier.clone();

        if matches!(self.state, StreamState::Init) {
            self.message_id = chunk.id.clone();
            self.model = chunk.model.clone();
            self.service_tier = match chunk_service_tier {
                Some(ChatCompletionServiceTier::Priority) => BetaServiceTier::Priority,
                _ => BetaServiceTier::Standard,
            };
            if let Some(usage) = chunk.usage.as_ref() {
                self.apply_usage(usage);
            }
            out.push(message_start_event(
                self.message_id.clone(),
                self.model.clone(),
                self.service_tier.clone(),
                self.input_tokens,
                self.cached_input_tokens,
            ));
            self.state = StreamState::Running;
        }

        if let Some(usage) = chunk.usage {
            self.apply_usage(&usage);
        }
        if matches!(
            chunk_service_tier,
            Some(ChatCompletionServiceTier::Priority)
        ) {
            self.service_tier = BetaServiceTier::Priority;
        }

        for choice in chunk.choices {
            let delta = choice.delta;

            if let Some(text) = delta.content {
                self.emit_text_block(&mut out, text);
            }

            if let Some(refusal) = delta.refusal {
                self.has_refusal = true;
                self.emit_text_block(&mut out, refusal);
                self.stop_reason = Some(BetaStopReason::Refusal);
            }

            if let Some(function_call) = delta.function_call {
                self.has_tool_use = true;
                let legacy_block_index = if let Some(index) = self.open_legacy_function_block {
                    index
                } else {
                    let index = self.next_block_index;
                    self.next_block_index = self.next_block_index.saturating_add(1);
                    out.push(start_tool_use_block_event(
                        index,
                        "function_call".to_string(),
                        function_call
                            .name
                            .clone()
                            .unwrap_or_else(|| "function_call".to_string()),
                    ));
                    self.open_legacy_function_block = Some(index);
                    index
                };

                if let Some(arguments) = function_call.arguments
                    && !arguments.is_empty()
                {
                    out.push(input_json_delta_event(legacy_block_index, arguments));
                }
            }

            if let Some(tool_calls) = delta.tool_calls {
                for tool_call in tool_calls {
                    self.has_tool_use = true;
                    let block_index =
                        if let Some(index) = self.open_tool_blocks.get(&tool_call.index) {
                            *index
                        } else {
                            let index = self.next_block_index;
                            self.next_block_index = self.next_block_index.saturating_add(1);
                            let name = tool_call
                                .function
                                .as_ref()
                                .and_then(|function| function.name.clone())
                                .unwrap_or_else(|| format!("tool_{}", tool_call.index));
                            let id = tool_call
                                .id
                                .clone()
                                .unwrap_or_else(|| format!("tool_call_{}", tool_call.index));
                            out.push(start_tool_use_block_event(index, id, name));
                            self.open_tool_blocks.insert(tool_call.index, index);
                            index
                        };

                    if let Some(function) = tool_call.function
                        && let Some(arguments) = function.arguments
                        && !arguments.is_empty()
                    {
                        out.push(input_json_delta_event(block_index, arguments));
                    }
                }
            }

            if let Some(finish_reason) = choice.finish_reason {
                self.stop_reason = Some(match finish_reason {
                    ChatCompletionFinishReason::Stop => BetaStopReason::EndTurn,
                    ChatCompletionFinishReason::Length => BetaStopReason::MaxTokens,
                    ChatCompletionFinishReason::ToolCalls
                    | ChatCompletionFinishReason::FunctionCall => BetaStopReason::ToolUse,
                    ChatCompletionFinishReason::ContentFilter => BetaStopReason::Refusal,
                });
            }
        }

        out
    }

    fn emit_text_block(&mut self, out: &mut Vec<ClaudeStreamEvent>, text: String) {
        let _ = push_text_block(out, &mut self.next_block_index, text);
    }

    pub fn finish(&mut self) -> Vec<ClaudeStreamEvent> {
        if self.is_finished() {
            return Vec::new();
        }

        let mut out = Vec::new();
        if matches!(self.state, StreamState::Init) {
            out.push(message_start_event(
                self.message_id.clone(),
                self.model.clone(),
                self.service_tier.clone(),
                self.input_tokens,
                self.cached_input_tokens,
            ));
            self.state = StreamState::Running;
        }

        for block_index in std::mem::take(&mut self.open_tool_blocks).into_values() {
            out.push(stop_block_event(block_index));
        }
        if let Some(block_index) = self.open_legacy_function_block.take() {
            out.push(stop_block_event(block_index));
        }

        let final_stop_reason = self.stop_reason.clone().or({
            if self.has_tool_use {
                Some(BetaStopReason::ToolUse)
            } else if self.has_refusal {
                Some(BetaStopReason::Refusal)
            } else {
                Some(BetaStopReason::EndTurn)
            }
        });
        out.push(message_delta_event(
            final_stop_reason,
            self.input_tokens,
            self.cached_input_tokens,
            self.output_tokens,
        ));
        out.push(message_stop_event());
        self.state = StreamState::Finished;
        out
    }
}
