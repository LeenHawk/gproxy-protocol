use std::collections::BTreeMap;

use crate::gemini::count_tokens::types::{GeminiContentRole, GeminiFunctionCall, GeminiPart};
use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;
use crate::gemini::generate_content::types::{
    GeminiBlockReason, GeminiCandidate, GeminiContent, GeminiFinishReason, GeminiPromptFeedback,
    GeminiUsageMetadata,
};
use crate::openai::create_chat_completions::stream::ChatCompletionChunk;
use crate::openai::create_chat_completions::types::ChatCompletionFinishReason;
use crate::transform::gemini::stream_generate_content::utils::parse_json_object_or_empty;

#[derive(Debug, Clone, Default)]
struct ToolCallState {
    id: String,
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Default)]
pub struct OpenAiChatCompletionsToGeminiStream {
    response_id: Option<String>,
    model_version: Option<String>,
    usage_metadata: Option<GeminiUsageMetadata>,
    legacy_function_name: String,
    legacy_function_arguments: String,
    tool_calls: BTreeMap<u32, ToolCallState>,
    finished: bool,
}

impl OpenAiChatCompletionsToGeminiStream {
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn finish_reason_from_openai(reason: ChatCompletionFinishReason) -> GeminiFinishReason {
        match reason {
            ChatCompletionFinishReason::Stop => GeminiFinishReason::Stop,
            ChatCompletionFinishReason::Length => GeminiFinishReason::MaxTokens,
            ChatCompletionFinishReason::ToolCalls | ChatCompletionFinishReason::FunctionCall => {
                GeminiFinishReason::UnexpectedToolCall
            }
            ChatCompletionFinishReason::ContentFilter => GeminiFinishReason::Safety,
        }
    }

    fn chunk_from_parts(
        &self,
        parts: Vec<GeminiPart>,
        index: u32,
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
                index: Some(index),
                ..GeminiCandidate::default()
            }]),
            prompt_feedback,
            usage_metadata: self.usage_metadata.clone(),
            model_version: self.model_version.clone(),
            response_id: self.response_id.clone(),
            model_status: None,
        }
    }

    fn text_chunk(&self, text: String, index: u32) -> Option<GeminiGenerateContentResponseBody> {
        if text.is_empty() {
            None
        } else {
            Some(self.chunk_from_parts(
                vec![GeminiPart {
                    text: Some(text),
                    ..GeminiPart::default()
                }],
                index,
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
        index: u32,
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
            index,
            None,
            None,
        )
    }

    pub fn on_chunk(
        &mut self,
        chunk: ChatCompletionChunk,
        out: &mut Vec<GeminiGenerateContentResponseBody>,
    ) {
        if self.finished {
            return;
        }

        self.response_id = Some(chunk.id);
        self.model_version = Some(chunk.model);
        if let Some(usage) = chunk.usage {
            self.usage_metadata = Some(GeminiUsageMetadata {
                prompt_token_count: Some(usage.prompt_tokens),
                cached_content_token_count: usage
                    .prompt_tokens_details
                    .and_then(|details| details.cached_tokens),
                candidates_token_count: Some(usage.completion_tokens),
                thoughts_token_count: usage
                    .completion_tokens_details
                    .and_then(|details| details.reasoning_tokens),
                total_token_count: Some(usage.total_tokens),
                ..GeminiUsageMetadata::default()
            });
        }

        for choice in chunk.choices {
            let index = choice.index;
            let delta = choice.delta;

            if let Some(content) = delta.content
                && let Some(chunk) = self.text_chunk(content, index)
            {
                out.push(chunk);
            }

            if let Some(refusal) = delta.refusal
                && let Some(chunk) = self.text_chunk(refusal, index)
            {
                out.push(chunk);
            }

            if let Some(function_call) = delta.function_call {
                if let Some(name) = function_call.name {
                    self.legacy_function_name = name;
                }
                if let Some(arguments) = function_call.arguments {
                    self.legacy_function_arguments.push_str(&arguments);
                }
                let name = if self.legacy_function_name.is_empty() {
                    "function_call".to_string()
                } else {
                    self.legacy_function_name.clone()
                };
                out.push(self.function_call_chunk(
                    "function_call".to_string(),
                    name,
                    self.legacy_function_arguments.clone(),
                    index,
                ));
            }

            if let Some(tool_calls) = delta.tool_calls {
                for tool_call in tool_calls {
                    let snapshot = {
                        let entry = self.tool_calls.entry(tool_call.index).or_insert_with(|| {
                            ToolCallState {
                                id: tool_call
                                    .id
                                    .clone()
                                    .unwrap_or_else(|| format!("tool_call_{}", tool_call.index)),
                                name: format!("tool_{}", tool_call.index),
                                arguments: String::new(),
                            }
                        });

                        if let Some(id) = tool_call.id {
                            entry.id = id;
                        }

                        if let Some(function) = tool_call.function {
                            if let Some(name) = function.name {
                                entry.name = name;
                            }
                            if let Some(arguments) = function.arguments {
                                entry.arguments.push_str(&arguments);
                            }
                        }

                        (
                            entry.id.clone(),
                            entry.name.clone(),
                            entry.arguments.clone(),
                        )
                    };

                    out.push(self.function_call_chunk(snapshot.0, snapshot.1, snapshot.2, index));
                }
            }

            if let Some(reason) = choice.finish_reason {
                let finish_reason = Self::finish_reason_from_openai(reason);
                let prompt_feedback = if matches!(finish_reason, GeminiFinishReason::Safety) {
                    Some(GeminiPromptFeedback {
                        block_reason: Some(GeminiBlockReason::Safety),
                        safety_ratings: None,
                    })
                } else {
                    None
                };
                out.push(self.chunk_from_parts(
                    Vec::new(),
                    index,
                    Some(finish_reason),
                    prompt_feedback,
                ));
            }
        }
    }

    pub fn finish(&mut self, out: &mut Vec<GeminiGenerateContentResponseBody>) {
        if !self.finished {
            self.finished = true;
            let _ = out;
        }
    }
}
