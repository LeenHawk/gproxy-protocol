use std::collections::BTreeMap;

use http::StatusCode;

use crate::openai::create_chat_completions::response::OpenAiChatCompletionsResponse;
use crate::openai::create_chat_completions::stream::ChatCompletionChunk;
use crate::openai::create_chat_completions::types as ct;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::utils::TransformError;

#[derive(Debug, Clone, Default)]
struct ToolCallAcc {
    id: Option<String>,
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Default)]
struct ChoiceAcc {
    content: String,
    reasoning_content: String,
    reasoning_details: Vec<ct::ChatCompletionReasoningDetail>,
    refusal: String,
    annotations: Vec<ct::ChatCompletionAnnotation>,
    has_function_call: bool,
    function_call_name: String,
    function_call_arguments: String,
    tool_calls: BTreeMap<u32, ToolCallAcc>,
    finish_reason: Option<ct::ChatCompletionFinishReason>,
    logprobs: Option<ct::ChatCompletionLogprobs>,
}

impl ChoiceAcc {
    fn into_choice(self, index: u32) -> ct::ChatCompletionChoice {
        let function_call = if self.has_function_call
            || !self.function_call_name.is_empty()
            || !self.function_call_arguments.is_empty()
        {
            Some(ct::ChatCompletionFunctionCall {
                name: if self.function_call_name.is_empty() {
                    "function".to_string()
                } else {
                    self.function_call_name
                },
                arguments: self.function_call_arguments,
            })
        } else {
            None
        };

        let tool_calls = if self.tool_calls.is_empty() {
            None
        } else {
            Some(
                self.tool_calls
                    .into_iter()
                    .map(|(tool_index, tool_call)| {
                        ct::ChatCompletionMessageToolCall::Function(
                            ct::ChatCompletionMessageFunctionToolCall {
                                id: tool_call
                                    .id
                                    .unwrap_or_else(|| format!("tool_call_{index}_{tool_index}")),
                                function: ct::ChatCompletionFunctionCall {
                                    name: if tool_call.name.is_empty() {
                                        "function".to_string()
                                    } else {
                                        tool_call.name
                                    },
                                    arguments: tool_call.arguments,
                                },
                                type_: ct::ChatCompletionMessageFunctionToolCallType::Function,
                            },
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        };

        let finish_reason = self.finish_reason.unwrap_or_else(|| {
            if function_call.is_some() || tool_calls.is_some() {
                ct::ChatCompletionFinishReason::ToolCalls
            } else {
                ct::ChatCompletionFinishReason::Stop
            }
        });

        ct::ChatCompletionChoice {
            finish_reason,
            index,
            logprobs: self.logprobs,
            message: ct::ChatCompletionMessage {
                content: if self.content.is_empty() {
                    None
                } else {
                    Some(self.content)
                },
                reasoning_content: if self.reasoning_content.is_empty() {
                    None
                } else {
                    Some(self.reasoning_content)
                },
                reasoning_details: if self.reasoning_details.is_empty() {
                    None
                } else {
                    Some(self.reasoning_details)
                },
                refusal: if self.refusal.is_empty() {
                    None
                } else {
                    Some(self.refusal)
                },
                role: ct::ChatCompletionAssistantRole::Assistant,
                annotations: if self.annotations.is_empty() {
                    None
                } else {
                    Some(self.annotations)
                },
                audio: None,
                function_call,
                tool_calls,
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct StreamAcc {
    choices: BTreeMap<u32, ChoiceAcc>,
    response_id: String,
    model: String,
    created: u64,
    service_tier: Option<ct::ChatCompletionServiceTier>,
    system_fingerprint: Option<String>,
    usage: Option<ct::CompletionUsage>,
}

fn apply_chunk(chunk: ChatCompletionChunk, acc: &mut StreamAcc) {
    acc.response_id = chunk.id;
    acc.model = chunk.model;
    acc.created = chunk.created;
    if chunk.service_tier.is_some() {
        acc.service_tier = chunk.service_tier;
    }
    if chunk.system_fingerprint.is_some() {
        acc.system_fingerprint = chunk.system_fingerprint;
    }
    if chunk.usage.is_some() {
        acc.usage = chunk.usage;
    }

    for choice in chunk.choices {
        let entry = acc.choices.entry(choice.index).or_default();
        if let Some(content) = choice.delta.content {
            entry.content.push_str(&content);
        }
        if let Some(reasoning_content) = choice.delta.reasoning_content {
            entry.reasoning_content.push_str(&reasoning_content);
        }
        if let Some(reasoning_details) = choice.delta.reasoning_details {
            entry.reasoning_details.extend(reasoning_details);
        }
        if let Some(refusal) = choice.delta.refusal {
            entry.refusal.push_str(&refusal);
        }
        if let Some(annotations) = choice.delta.annotations {
            entry.annotations.extend(annotations);
        }

        if let Some(function_call) = choice.delta.function_call {
            entry.has_function_call = true;
            if let Some(name) = function_call.name
                && !name.is_empty()
            {
                entry.function_call_name = name;
            }
            if let Some(arguments) = function_call.arguments {
                entry.function_call_arguments.push_str(&arguments);
            }
        }

        if let Some(tool_calls) = choice.delta.tool_calls {
            for tool_call in tool_calls {
                let tool_entry = entry.tool_calls.entry(tool_call.index).or_default();
                if let Some(id) = tool_call.id {
                    tool_entry.id = Some(id);
                }
                if let Some(function) = tool_call.function {
                    if let Some(name) = function.name
                        && !name.is_empty()
                    {
                        tool_entry.name = name;
                    }
                    if let Some(arguments) = function.arguments {
                        tool_entry.arguments.push_str(&arguments);
                    }
                }
            }
        }

        if choice.finish_reason.is_some() {
            entry.finish_reason = choice.finish_reason;
        }
        if choice.logprobs.is_some() {
            entry.logprobs = choice.logprobs;
        }
    }
}

impl TryFrom<Vec<ChatCompletionChunk>> for OpenAiChatCompletionsResponse {
    type Error = TransformError;

    fn try_from(value: Vec<ChatCompletionChunk>) -> Result<Self, TransformError> {
        let mut acc = StreamAcc::default();
        let mut saw_chunk = false;

        for chunk in value {
            saw_chunk = true;
            apply_chunk(chunk, &mut acc);
        }

        if !saw_chunk {
            return Err(TransformError::not_implemented(
                "cannot convert empty OpenAI chat SSE stream body to non-stream response",
            ));
        }

        let choices_map = std::mem::take(&mut acc.choices);
        let choices = if choices_map.is_empty() {
            vec![ChoiceAcc::default().into_choice(0)]
        } else {
            choices_map
                .into_iter()
                .map(|(index, choice)| choice.into_choice(index))
                .collect::<Vec<_>>()
        };

        Ok(OpenAiChatCompletionsResponse::Success {
            stats_code: StatusCode::OK,
            headers: OpenAiResponseHeaders::default(),
            body: ct::ChatCompletion {
                id: if acc.response_id.is_empty() {
                    "chatcmpl".to_string()
                } else {
                    acc.response_id
                },
                choices,
                created: acc.created,
                model: if acc.model.is_empty() {
                    "gpt".to_string()
                } else {
                    acc.model
                },
                object: ct::ChatCompletionObject::ChatCompletion,
                service_tier: acc.service_tier,
                system_fingerprint: acc.system_fingerprint,
                usage: acc.usage,
            },
        })
    }
}
