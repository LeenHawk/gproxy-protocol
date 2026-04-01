use crate::claude::create_message::stream::ClaudeStreamEvent;
use crate::claude::create_message::types::{BetaServiceTier, BetaStopReason};
use crate::gemini::count_tokens::types::{GeminiLanguage, GeminiOutcome};
use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;
use crate::gemini::generate_content::types::{GeminiBlockReason, GeminiFinishReason};
use crate::gemini::stream_generate_content::response::GeminiStreamGenerateContentResponse;
use crate::transform::claude::stream_generate_content::utils::{
    message_delta_event, message_start_event, message_stop_event, push_text_block,
    push_thinking_block, push_tool_use_block, stream_error_event,
};
use crate::transform::utils::TransformError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamState {
    Init,
    Running,
    Finished,
}

#[derive(Debug, Clone)]
pub struct GeminiToClaudeStream {
    state: StreamState,
    next_block_index: u64,
    chunk_seq: u64,
    message_id: String,
    model: String,
    input_tokens: u64,
    cached_input_tokens: u64,
    output_tokens: u64,
    stop_reason: Option<BetaStopReason>,
    has_tool_use: bool,
    has_refusal: bool,
}

impl Default for GeminiToClaudeStream {
    fn default() -> Self {
        Self {
            state: StreamState::Init,
            next_block_index: 0,
            chunk_seq: 0,
            message_id: String::new(),
            model: String::new(),
            input_tokens: 0,
            cached_input_tokens: 0,
            output_tokens: 0,
            stop_reason: None,
            has_tool_use: false,
            has_refusal: false,
        }
    }
}

impl GeminiToClaudeStream {
    pub fn is_finished(&self) -> bool {
        matches!(self.state, StreamState::Finished)
    }

    fn update_envelope_from_chunk(&mut self, chunk: &GeminiGenerateContentResponseBody) {
        if let Some(response_id) = chunk.response_id.as_ref() {
            self.message_id = response_id.clone();
        }
        if let Some(model_version) = chunk.model_version.as_ref() {
            self.model = model_version.clone();
        }
        if let Some(usage_metadata) = chunk.usage_metadata.as_ref() {
            let prompt_input_tokens = usage_metadata
                .prompt_token_count
                .unwrap_or(0)
                .saturating_add(usage_metadata.tool_use_prompt_token_count.unwrap_or(0));
            let cached_tokens = usage_metadata.cached_content_token_count.unwrap_or(0);
            let output_tokens = usage_metadata
                .candidates_token_count
                .unwrap_or(0)
                .saturating_add(usage_metadata.thoughts_token_count.unwrap_or(0));
            let total_input_tokens = usage_metadata
                .total_token_count
                .map(|total| total.saturating_sub(output_tokens))
                .unwrap_or_else(|| prompt_input_tokens.saturating_add(cached_tokens));

            self.input_tokens = total_input_tokens.saturating_sub(cached_tokens);
            self.cached_input_tokens = cached_tokens;
            self.output_tokens = output_tokens;
        }
    }

    fn ensure_running(&mut self, out: &mut Vec<ClaudeStreamEvent>) {
        if matches!(self.state, StreamState::Init) {
            out.push(message_start_event(
                self.message_id.clone(),
                self.model.clone(),
                BetaServiceTier::Standard,
                self.input_tokens,
                self.cached_input_tokens,
            ));
            self.state = StreamState::Running;
        }
    }

    fn emit_text_block(&mut self, out: &mut Vec<ClaudeStreamEvent>, text: String) {
        self.ensure_running(out);
        let _ = push_text_block(out, &mut self.next_block_index, text);
    }

    fn emit_thinking_block(
        &mut self,
        out: &mut Vec<ClaudeStreamEvent>,
        signature: String,
        thinking: String,
    ) {
        self.ensure_running(out);
        let _ = push_thinking_block(out, &mut self.next_block_index, signature, thinking);
    }

    fn emit_tool_use_block(
        &mut self,
        out: &mut Vec<ClaudeStreamEvent>,
        id: String,
        name: String,
        input_json: Option<String>,
    ) {
        self.ensure_running(out);
        self.has_tool_use = true;
        let _ = push_tool_use_block(out, &mut self.next_block_index, id, name, input_json);
    }


    pub fn on_chunk(
        &mut self,
        chunk: GeminiGenerateContentResponseBody,
        out: &mut Vec<ClaudeStreamEvent>,
    ) {
        if self.is_finished() {
            return;
        }

        self.update_envelope_from_chunk(&chunk);
        let chunk_index = self.chunk_seq;
        self.chunk_seq = self.chunk_seq.saturating_add(1);

        let mut chunk_has_content = false;

        if let Some(status_message) = chunk
            .model_status
            .as_ref()
            .and_then(|status| status.message.as_ref())
            && !status_message.is_empty()
        {
            chunk_has_content = true;
            self.emit_text_block(out, format!("model_status: {status_message}"));
        }

        if let Some(candidates) = chunk.candidates {
            for (candidate_index, candidate) in candidates.into_iter().enumerate() {
                let mut candidate_has_content = false;
                if let Some(content) = candidate.content {
                    for (part_index, part) in content.parts.into_iter().enumerate() {
                        if part.thought.unwrap_or(false) {
                            if let Some(thinking) = part.text {
                                candidate_has_content = true;
                                chunk_has_content = true;
                                self.emit_thinking_block(
                                    out,
                                    part.thought_signature.unwrap_or_else(|| {
                                        format!(
                                            "thought_{chunk_index}_{candidate_index}_{part_index}"
                                        )
                                    }),
                                    thinking,
                                );
                            }
                        } else if let Some(text) = part.text {
                            candidate_has_content = true;
                            chunk_has_content = true;
                            self.emit_text_block(out, text);
                        }

                        if let Some(inline_data) = part.inline_data {
                            candidate_has_content = true;
                            chunk_has_content = true;
                            self.emit_text_block(
                                out,
                                format!(
                                    "inline_data({}): {}",
                                    inline_data.mime_type, inline_data.data
                                ),
                            );
                        }

                        if let Some(function_call) = part.function_call {
                            candidate_has_content = true;
                            chunk_has_content = true;
                            self.emit_tool_use_block(
                                out,
                                function_call.id.unwrap_or_else(|| {
                                    format!(
                                        "tool_call_{chunk_index}_{candidate_index}_{part_index}"
                                    )
                                }),
                                function_call.name,
                                function_call
                                    .args
                                    .and_then(|args| serde_json::to_string(&args).ok()),
                            );
                        }

                        if let Some(function_response) = part.function_response {
                            if let Ok(response_json) =
                                serde_json::to_string(&function_response.response)
                                && !response_json.is_empty()
                            {
                                candidate_has_content = true;
                                chunk_has_content = true;
                                self.emit_text_block(
                                    out,
                                    format!(
                                        "function_response({}): {response_json}",
                                        function_response.name
                                    ),
                                );
                            }
                            if let Some(parts) = function_response.parts {
                                for (response_part_index, response_part) in
                                    parts.into_iter().enumerate()
                                {
                                    if let Some(inline_data) = response_part.inline_data {
                                        candidate_has_content = true;
                                        chunk_has_content = true;
                                        self.emit_text_block(
                                            out,
                                            format!(
                                                "function_response.inline_data({candidate_index}:{part_index}:{response_part_index})({}): {}",
                                                inline_data.mime_type, inline_data.data
                                            ),
                                        );
                                    }
                                }
                            }
                        }

                        if let Some(executable_code) = part.executable_code {
                            let language = match executable_code.language {
                                GeminiLanguage::LanguageUnspecified => "unspecified",
                                GeminiLanguage::Python => "python",
                            };
                            candidate_has_content = true;
                            chunk_has_content = true;
                            self.emit_text_block(
                                out,
                                format!("executable_code({language}): {}", executable_code.code),
                            );
                        }

                        if let Some(code_execution_result) = part.code_execution_result {
                            let outcome = match code_execution_result.outcome {
                                GeminiOutcome::OutcomeUnspecified => "unspecified",
                                GeminiOutcome::OutcomeOk => "ok",
                                GeminiOutcome::OutcomeFailed => "failed",
                                GeminiOutcome::OutcomeDeadlineExceeded => "deadline_exceeded",
                            };
                            let output_text = code_execution_result.output.unwrap_or_default();
                            candidate_has_content = true;
                            chunk_has_content = true;
                            if output_text.is_empty() {
                                self.emit_text_block(
                                    out,
                                    format!("code_execution_result({outcome})"),
                                );
                            } else {
                                self.emit_text_block(
                                    out,
                                    format!("code_execution_result({outcome}): {output_text}"),
                                );
                            }
                        }

                        if let Some(file_data) = part.file_data {
                            candidate_has_content = true;
                            chunk_has_content = true;
                            if let Some(mime_type) = file_data.mime_type {
                                self.emit_text_block(
                                    out,
                                    format!("file_data({mime_type}): {}", file_data.file_uri),
                                );
                            } else {
                                self.emit_text_block(out, file_data.file_uri);
                            }
                        }
                    }
                }

                if !candidate_has_content
                    && let Some(finish_message) = candidate.finish_message
                    && !finish_message.is_empty()
                {
                    chunk_has_content = true;
                    self.emit_text_block(out, finish_message);
                }

                if let Some(reason) = candidate.finish_reason {
                    self.stop_reason = Some(match reason {
                        GeminiFinishReason::MaxTokens => BetaStopReason::MaxTokens,
                        GeminiFinishReason::MalformedFunctionCall
                        | GeminiFinishReason::UnexpectedToolCall
                        | GeminiFinishReason::TooManyToolCalls
                        | GeminiFinishReason::MissingThoughtSignature => BetaStopReason::ToolUse,
                        GeminiFinishReason::Safety
                        | GeminiFinishReason::Recitation
                        | GeminiFinishReason::Blocklist
                        | GeminiFinishReason::ProhibitedContent
                        | GeminiFinishReason::Spii
                        | GeminiFinishReason::ImageSafety
                        | GeminiFinishReason::ImageProhibitedContent
                        | GeminiFinishReason::ImageRecitation => {
                            self.has_refusal = true;
                            BetaStopReason::Refusal
                        }
                        GeminiFinishReason::Stop
                        | GeminiFinishReason::FinishReasonUnspecified
                        | GeminiFinishReason::Language
                        | GeminiFinishReason::Other
                        | GeminiFinishReason::ImageOther
                        | GeminiFinishReason::NoImage => BetaStopReason::EndTurn,
                    });
                }
            }
        } else {
            self.stop_reason = Some(
                match chunk
                    .prompt_feedback
                    .as_ref()
                    .and_then(|feedback| feedback.block_reason.as_ref())
                {
                    Some(GeminiBlockReason::Safety)
                    | Some(GeminiBlockReason::Blocklist)
                    | Some(GeminiBlockReason::ProhibitedContent)
                    | Some(GeminiBlockReason::ImageSafety) => {
                        self.has_refusal = true;
                        BetaStopReason::Refusal
                    }
                    Some(GeminiBlockReason::BlockReasonUnspecified)
                    | Some(GeminiBlockReason::Other)
                    | None => BetaStopReason::EndTurn,
                },
            );
        }

        if !chunk_has_content {
            self.ensure_running(out);
        }
    }

    pub fn finish(&mut self, out: &mut Vec<ClaudeStreamEvent>) {
        if self.is_finished() {
            return;
        }

        self.ensure_running(out);

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
    }
}

impl TryFrom<GeminiStreamGenerateContentResponse> for Vec<ClaudeStreamEvent> {
    type Error = TransformError;

    fn try_from(value: GeminiStreamGenerateContentResponse) -> Result<Self, TransformError> {
        match value {
            GeminiStreamGenerateContentResponse::Success { .. } => {
                // The new response type no longer contains chunks inline;
                // chunks are processed individually via on_chunk().
                Ok(Vec::new())
            }
            GeminiStreamGenerateContentResponse::Error { body, .. } => {
                Ok(vec![stream_error_event(body.error.message)])
            }
        }
    }
}
