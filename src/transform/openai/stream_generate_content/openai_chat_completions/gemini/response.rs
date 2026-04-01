use std::collections::{BTreeMap, BTreeSet};

use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;
use crate::gemini::generate_content::types::{
    GeminiBlockReason, GeminiCandidate, GeminiFinishReason,
};
use crate::openai::create_chat_completions::stream::{
    ChatCompletionChunk, ChatCompletionChunkChoice, ChatCompletionChunkDelta,
    ChatCompletionChunkDeltaToolCall, ChatCompletionChunkDeltaToolCallType,
    ChatCompletionFunctionCallDelta,
};
use crate::openai::create_chat_completions::types as ct;
use crate::transform::openai::generate_content::openai_chat_completions::gemini::utils::{
    gemini_citation_annotations, gemini_function_response_to_text, gemini_logprobs,
    json_object_to_string, prompt_feedback_refusal_text,
};
use crate::transform::openai::model_list::gemini::utils::strip_models_prefix;

#[derive(Debug, Clone)]
struct OpenAiChatToolState {
    choice_index: u32,
    tool_index: u32,
    call_id: String,
    name: String,
    name_emitted: bool,
    arguments_snapshot: String,
}

#[derive(Debug, Default, Clone)]
pub struct GeminiToOpenAiChatCompletionsStream {
    response_id: String,
    model: String,
    created: u64,
    input_tokens: u64,
    cached_input_tokens: u64,
    output_tokens: u64,
    reasoning_tokens: u64,
    incomplete_finish_reason: Option<ct::ChatCompletionFinishReason>,
    choice_finish_reasons: BTreeMap<u32, ct::ChatCompletionFinishReason>,
    output_choice_map: BTreeMap<u64, u32>,
    role_emitted: BTreeSet<u32>,
    choice_tool_counts: BTreeMap<u32, u32>,
    choice_has_tool_calls: BTreeSet<u32>,
    tool_states: BTreeMap<String, OpenAiChatToolState>,
    chunk_sequence: u64,
    finished: bool,
}

impl GeminiToOpenAiChatCompletionsStream {
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn fallback_response_id(&self) -> String {
        if self.response_id.is_empty() {
            "response".to_string()
        } else {
            self.response_id.clone()
        }
    }

    fn fallback_model(&self) -> String {
        if self.model.is_empty() {
            "gemini".to_string()
        } else {
            self.model.clone()
        }
    }

    fn usage(&self) -> Option<ct::CompletionUsage> {
        if self.input_tokens == 0
            && self.cached_input_tokens == 0
            && self.output_tokens == 0
            && self.reasoning_tokens == 0
        {
            return None;
        }

        Some(ct::CompletionUsage {
            completion_tokens: self.output_tokens,
            prompt_tokens: self.input_tokens,
            total_tokens: self.input_tokens.saturating_add(self.output_tokens),
            completion_tokens_details: Some(ct::CompletionTokensDetails {
                accepted_prediction_tokens: None,
                audio_tokens: None,
                reasoning_tokens: Some(self.reasoning_tokens),
                rejected_prediction_tokens: None,
            }),
            prompt_tokens_details: Some(ct::PromptTokensDetails {
                audio_tokens: None,
                cached_tokens: Some(self.cached_input_tokens),
            }),
        })
    }

    fn make_chunk(
        &self,
        index: u32,
        delta: ChatCompletionChunkDelta,
        finish_reason: Option<ct::ChatCompletionFinishReason>,
        usage: Option<ct::CompletionUsage>,
        logprobs: Option<ct::ChatCompletionLogprobs>,
    ) -> ChatCompletionChunk {
        ChatCompletionChunk {
            id: self.fallback_response_id(),
            choices: vec![ChatCompletionChunkChoice {
                delta,
                finish_reason,
                index,
                logprobs,
            }],
            created: self.created,
            model: self.fallback_model(),
            object: crate::openai::create_chat_completions::stream::ChatCompletionChunkObject::ChatCompletionChunk,
            service_tier: None,
            system_fingerprint: None,
            usage,
        }
    }

    fn ensure_choice_index(&mut self, output_index: u64) -> u32 {
        if let Some(choice_index) = self.output_choice_map.get(&output_index) {
            return *choice_index;
        }

        let choice_index = u32::try_from(self.output_choice_map.len()).unwrap_or(u32::MAX);
        self.output_choice_map.insert(output_index, choice_index);
        choice_index
    }

    fn maybe_emit_role(&mut self, out: &mut Vec<ChatCompletionChunk>, choice_index: u32) {
        if self.role_emitted.insert(choice_index) {
            out.push(self.make_chunk(
                choice_index,
                ChatCompletionChunkDelta {
                    role: Some(ct::ChatCompletionDeltaRole::Assistant),
                    ..Default::default()
                },
                None,
                None,
                None,
            ));
        }
    }

    fn emit_content(
        &mut self,
        output_index: u64,
        text: String,
        refusal: bool,
        out: &mut Vec<ChatCompletionChunk>,
    ) {
        let choice_index = self.ensure_choice_index(output_index);
        self.maybe_emit_role(out, choice_index);

        if text.is_empty() {
            return;
        }

        out.push(self.make_chunk(
            choice_index,
            ChatCompletionChunkDelta {
                content: if refusal { None } else { Some(text.clone()) },
                refusal: if refusal { Some(text) } else { None },
                ..Default::default()
            },
            None,
            None,
            None,
        ));
    }

    fn emit_annotations(
        &mut self,
        output_index: u64,
        annotations: Vec<ct::ChatCompletionAnnotation>,
        out: &mut Vec<ChatCompletionChunk>,
    ) {
        if annotations.is_empty() {
            return;
        }

        let choice_index = self.ensure_choice_index(output_index);
        self.maybe_emit_role(out, choice_index);
        out.push(self.make_chunk(
            choice_index,
            ChatCompletionChunkDelta {
                annotations: Some(annotations),
                ..Default::default()
            },
            None,
            None,
            None,
        ));
    }

    fn emit_logprobs(
        &mut self,
        output_index: u64,
        logprobs: ct::ChatCompletionLogprobs,
        out: &mut Vec<ChatCompletionChunk>,
    ) {
        let choice_index = self.ensure_choice_index(output_index);
        self.maybe_emit_role(out, choice_index);
        out.push(self.make_chunk(choice_index, Default::default(), None, None, Some(logprobs)));
    }

    pub fn emit_error_refusal(&mut self, text: String, out: &mut Vec<ChatCompletionChunk>) {
        self.emit_content(0, text, true, out);
    }

    fn emit_function_call_snapshot(
        &mut self,
        output_index: u64,
        call_id: String,
        name: String,
        arguments_snapshot: String,
        out: &mut Vec<ChatCompletionChunk>,
    ) {
        if let Some(state) = self.tool_states.get_mut(&call_id) {
            if !name.is_empty() {
                state.name = name;
            }

            let delta = if arguments_snapshot.starts_with(&state.arguments_snapshot) {
                arguments_snapshot[state.arguments_snapshot.len()..].to_string()
            } else {
                arguments_snapshot.clone()
            };
            state.arguments_snapshot = arguments_snapshot;

            if delta.is_empty() {
                return;
            }

            let state_snapshot = state.clone();
            self.maybe_emit_role(out, state_snapshot.choice_index);
            out.push(self.make_chunk(
                state_snapshot.choice_index,
                ChatCompletionChunkDelta {
                    tool_calls: Some(vec![ChatCompletionChunkDeltaToolCall {
                        index: state_snapshot.tool_index,
                        id: Some(state_snapshot.call_id.clone()),
                        function: Some(ChatCompletionFunctionCallDelta {
                            name: if state_snapshot.name_emitted {
                                None
                            } else {
                                Some(state_snapshot.name.clone())
                            },
                            arguments: Some(delta),
                        }),
                        type_: Some(ChatCompletionChunkDeltaToolCallType::Function),
                    }]),
                    ..Default::default()
                },
                None,
                None,
                None,
            ));

            if let Some(tool_state) = self.tool_states.get_mut(&call_id) {
                tool_state.name_emitted = true;
            }
            return;
        }

        let choice_index = self.ensure_choice_index(output_index);
        self.maybe_emit_role(out, choice_index);

        let tool_index_ref = self.choice_tool_counts.entry(choice_index).or_insert(0);
        let tool_index = *tool_index_ref;
        *tool_index_ref = tool_index.saturating_add(1);
        self.choice_has_tool_calls.insert(choice_index);

        let state = OpenAiChatToolState {
            choice_index,
            tool_index,
            call_id: call_id.clone(),
            name,
            name_emitted: false,
            arguments_snapshot: arguments_snapshot.clone(),
        };
        self.tool_states.insert(call_id.clone(), state.clone());

        out.push(self.make_chunk(
            choice_index,
            ChatCompletionChunkDelta {
                tool_calls: Some(vec![ChatCompletionChunkDeltaToolCall {
                    index: state.tool_index,
                    id: Some(state.call_id.clone()),
                    function: Some(ChatCompletionFunctionCallDelta {
                        name: Some(state.name.clone()),
                        arguments: None,
                    }),
                    type_: Some(ChatCompletionChunkDeltaToolCallType::Function),
                }]),
                ..Default::default()
            },
            None,
            None,
            None,
        ));

        if let Some(tool_state) = self.tool_states.get_mut(&call_id) {
            tool_state.name_emitted = true;
        }

        if !arguments_snapshot.is_empty() && arguments_snapshot != "{}" {
            out.push(self.make_chunk(
                choice_index,
                ChatCompletionChunkDelta {
                    tool_calls: Some(vec![ChatCompletionChunkDeltaToolCall {
                        index: state.tool_index,
                        id: Some(state.call_id.clone()),
                        function: Some(ChatCompletionFunctionCallDelta {
                            name: None,
                            arguments: Some(arguments_snapshot),
                        }),
                        type_: Some(ChatCompletionChunkDeltaToolCallType::Function),
                    }]),
                    ..Default::default()
                },
                None,
                None,
                None,
            ));
        }
    }

    fn update_envelope_from_chunk(&mut self, chunk: &GeminiGenerateContentResponseBody) {
        if let Some(response_id) = chunk.response_id.as_ref() {
            self.response_id = response_id.clone();
        }
        if let Some(model_version) = chunk.model_version.as_ref() {
            self.model = strip_models_prefix(model_version);
        }
        if let Some(usage) = chunk.usage_metadata.as_ref() {
            self.input_tokens = usage
                .prompt_token_count
                .unwrap_or(0)
                .saturating_add(usage.tool_use_prompt_token_count.unwrap_or(0));
            self.cached_input_tokens = usage.cached_content_token_count.unwrap_or(0);
            self.output_tokens = usage
                .candidates_token_count
                .unwrap_or(0)
                .saturating_add(usage.thoughts_token_count.unwrap_or(0));
            self.reasoning_tokens = usage.thoughts_token_count.unwrap_or(0);
        }
    }

    fn map_finish_reason(reason: &GeminiFinishReason) -> ct::ChatCompletionFinishReason {
        match reason {
            GeminiFinishReason::MaxTokens => ct::ChatCompletionFinishReason::Length,
            GeminiFinishReason::Safety
            | GeminiFinishReason::Recitation
            | GeminiFinishReason::Blocklist
            | GeminiFinishReason::ProhibitedContent
            | GeminiFinishReason::Spii
            | GeminiFinishReason::ImageSafety
            | GeminiFinishReason::ImageProhibitedContent
            | GeminiFinishReason::ImageRecitation => ct::ChatCompletionFinishReason::ContentFilter,
            GeminiFinishReason::MalformedFunctionCall
            | GeminiFinishReason::UnexpectedToolCall
            | GeminiFinishReason::TooManyToolCalls => ct::ChatCompletionFinishReason::ToolCalls,
            GeminiFinishReason::Stop
            | GeminiFinishReason::FinishReasonUnspecified
            | GeminiFinishReason::Language
            | GeminiFinishReason::Other
            | GeminiFinishReason::ImageOther
            | GeminiFinishReason::NoImage
            | GeminiFinishReason::MissingThoughtSignature => ct::ChatCompletionFinishReason::Stop,
        }
    }

    fn map_block_reason(reason: &GeminiBlockReason) -> Option<ct::ChatCompletionFinishReason> {
        match reason {
            GeminiBlockReason::Safety
            | GeminiBlockReason::Blocklist
            | GeminiBlockReason::ProhibitedContent
            | GeminiBlockReason::ImageSafety => Some(ct::ChatCompletionFinishReason::ContentFilter),
            _ => None,
        }
    }

    pub fn on_chunk(
        &mut self,
        chunk: GeminiGenerateContentResponseBody,
        out: &mut Vec<ChatCompletionChunk>,
    ) {
        if self.finished {
            return;
        }

        self.update_envelope_from_chunk(&chunk);

        if let Some(reason) = chunk
            .prompt_feedback
            .as_ref()
            .and_then(|feedback| feedback.block_reason.as_ref())
            .and_then(Self::map_block_reason)
        {
            self.incomplete_finish_reason = Some(reason);
        }

        if let Some(refusal_text) = prompt_feedback_refusal_text(chunk.prompt_feedback.as_ref())
            && !refusal_text.is_empty()
        {
            self.emit_content(0, refusal_text, true, out);
        }

        if let Some(model_status_message) = chunk
            .model_status
            .as_ref()
            .and_then(|status| status.message.as_ref())
            && !model_status_message.is_empty()
        {
            self.emit_content(
                0,
                format!("model_status: {model_status_message}"),
                false,
                out,
            );
        }

        if let Some(candidates) = chunk.candidates {
            for (candidate_pos, candidate) in candidates.into_iter().enumerate() {
                let output_index = candidate.index.unwrap_or(candidate_pos as u32) as u64;
                self.process_candidate(output_index, candidate, out);
            }
        }

        self.chunk_sequence = self.chunk_sequence.saturating_add(1);
    }

    fn process_candidate(
        &mut self,
        output_index: u64,
        candidate: GeminiCandidate,
        out: &mut Vec<ChatCompletionChunk>,
    ) {
        let choice_index = self.ensure_choice_index(output_index);
        let GeminiCandidate {
            content,
            finish_reason,
            citation_metadata,
            logprobs_result,
            finish_message,
            ..
        } = candidate;

        if let Some(content) = content {
            for (part_index, part) in content.parts.into_iter().enumerate() {
                if part.thought.unwrap_or(false) {
                    continue;
                }

                if let Some(function_call) = part.function_call {
                    let call_id = function_call.id.unwrap_or_else(|| {
                        format!(
                            "tool_call_{}_{}_{}",
                            output_index, self.chunk_sequence, part_index
                        )
                    });
                    let arguments_snapshot = function_call
                        .args
                        .as_ref()
                        .map(json_object_to_string)
                        .unwrap_or_else(|| "{}".to_string());
                    self.emit_function_call_snapshot(
                        output_index,
                        call_id,
                        function_call.name,
                        arguments_snapshot,
                        out,
                    );
                    continue;
                }

                if let Some(function_response) = part.function_response {
                    self.emit_content(
                        output_index,
                        gemini_function_response_to_text(function_response),
                        false,
                        out,
                    );
                    continue;
                }

                if let Some(executable_code) = part.executable_code {
                    self.emit_content(output_index, executable_code.code, false, out);
                    continue;
                }

                if let Some(code_execution_result) = part.code_execution_result {
                    if let Some(output_text) = code_execution_result.output {
                        self.emit_content(output_index, output_text, false, out);
                    }
                    continue;
                }

                if let Some(text) = part.text {
                    self.emit_content(output_index, text, false, out);
                    continue;
                }

                if let Some(inline_data) = part.inline_data {
                    self.emit_content(
                        output_index,
                        format!("data:{};base64,{}", inline_data.mime_type, inline_data.data),
                        false,
                        out,
                    );
                    continue;
                }

                if let Some(file_data) = part.file_data {
                    self.emit_content(output_index, file_data.file_uri, false, out);
                }
            }
        }

        if let Some(finish_message) = finish_message
            && !finish_message.is_empty()
        {
            self.emit_content(output_index, finish_message, false, out);
        }

        let annotations = gemini_citation_annotations(citation_metadata.as_ref());
        self.emit_annotations(output_index, annotations, out);

        if let Some(logprobs) = gemini_logprobs(logprobs_result.as_ref()) {
            self.emit_logprobs(output_index, logprobs, out);
        }

        if let Some(finish_reason) = finish_reason.as_ref() {
            self.choice_finish_reasons
                .insert(choice_index, Self::map_finish_reason(finish_reason));
        }
    }

    pub fn finish(&mut self, out: &mut Vec<ChatCompletionChunk>) {
        if self.finished {
            return;
        }

        let default_reason = self
            .incomplete_finish_reason
            .clone()
            .unwrap_or(ct::ChatCompletionFinishReason::Stop);

        let mut choices = self.output_choice_map.values().copied().collect::<Vec<_>>();
        choices.sort_unstable();
        choices.dedup();
        if choices.is_empty() {
            choices.push(0);
        }

        for choice_index in &choices {
            let finish_reason = self
                .choice_finish_reasons
                .get(choice_index)
                .cloned()
                .or_else(|| {
                    if self.choice_has_tool_calls.contains(choice_index) {
                        Some(ct::ChatCompletionFinishReason::ToolCalls)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| default_reason.clone());
            out.push(self.make_chunk(
                *choice_index,
                Default::default(),
                Some(finish_reason),
                None,
                None,
            ));
        }

        if let Some(last) = out.last_mut() {
            last.usage = self.usage();
        }

        self.finished = true;
    }
}
