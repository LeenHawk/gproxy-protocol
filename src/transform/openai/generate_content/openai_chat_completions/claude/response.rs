use super::utils::{server_tool_name, stdout_stderr_text};
use crate::claude::count_tokens::types as cct;
use crate::claude::create_message::response::ClaudeCreateMessageResponse;
use crate::claude::create_message::types::{
    BetaStopReason, BetaTextCitation, BetaWebFetchToolResultBlockContent,
    BetaWebSearchToolResultBlockContent,
};
use crate::openai::create_chat_completions::response::OpenAiChatCompletionsResponse;
use crate::openai::create_chat_completions::types as oct;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::claude::utils::claude_model_to_string;
use crate::transform::openai::model_list::claude::utils::openai_error_response_from_claude;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeCreateMessageResponse> for OpenAiChatCompletionsResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeCreateMessageResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeCreateMessageResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let stop_reason = body.stop_reason.clone();
                let usage = body.usage;
                let service_tier = Some(match usage.service_tier.clone() {
                    crate::claude::create_message::types::BetaServiceTier::Standard => {
                        oct::ChatCompletionServiceTier::Default
                    }
                    crate::claude::create_message::types::BetaServiceTier::Priority => {
                        oct::ChatCompletionServiceTier::Priority
                    }
                    crate::claude::create_message::types::BetaServiceTier::Batch => {
                        oct::ChatCompletionServiceTier::Flex
                    }
                });

                let mut content_parts = Vec::new();
                let mut refusal_parts = Vec::new();
                let mut annotations = Vec::new();
                let mut function_tool_calls = Vec::new();
                let mut custom_tool_calls = Vec::new();

                for block in body.content {
                    match block {
                        crate::claude::create_message::types::BetaContentBlock::Text(block) => {
                            if !block.text.is_empty() {
                                content_parts.push(block.text);
                            }
                            if let Some(citations) = block.citations.as_ref() {
                                annotations.extend(
                                    citations
                                        .iter()
                                        .filter_map(|citation| match citation {
                                            BetaTextCitation::WebSearchResultLocation(value) => {
                                                Some(oct::ChatCompletionAnnotation {
                                                    type_: oct::ChatCompletionAnnotationType::UrlCitation,
                                                    url_citation: oct::ChatCompletionUrlCitation {
                                                        start_index: 0,
                                                        end_index: 0,
                                                        title: value.title.clone(),
                                                        url: value.url.clone(),
                                                    },
                                                })
                                            }
                                            BetaTextCitation::SearchResultLocation(value)
                                                if value.source.starts_with("http://")
                                                    || value.source.starts_with("https://") =>
                                            {
                                                Some(oct::ChatCompletionAnnotation {
                                                    type_: oct::ChatCompletionAnnotationType::UrlCitation,
                                                    url_citation: oct::ChatCompletionUrlCitation {
                                                        start_index: value.start_block_index,
                                                        end_index: value.end_block_index,
                                                        title: value.title.clone(),
                                                        url: value.source.clone(),
                                                    },
                                                })
                                            }
                                            _ => None,
                                        }),
                                );
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::ToolUse(block) => {
                            function_tool_calls.push(oct::ChatCompletionMessageFunctionToolCall {
                                id: block.id,
                                function: oct::ChatCompletionFunctionCall {
                                    arguments: serde_json::to_string(&block.input)
                                        .unwrap_or_else(|_| "{}".to_string()),
                                    name: block.name,
                                },
                                type_: oct::ChatCompletionMessageFunctionToolCallType::Function,
                            });
                        }
                        crate::claude::create_message::types::BetaContentBlock::ServerToolUse(block) => {
                            custom_tool_calls.push(oct::ChatCompletionMessageCustomToolCall {
                                id: block.id,
                                custom: oct::ChatCompletionMessageCustomToolCallPayload {
                                    input: serde_json::to_string(&block.input)
                                        .unwrap_or_else(|_| "{}".to_string()),
                                    name: server_tool_name(&block.name),
                                },
                                type_: oct::ChatCompletionMessageCustomToolCallType::Custom,
                            });
                        }
                        crate::claude::create_message::types::BetaContentBlock::McpToolUse(block) => {
                            custom_tool_calls.push(oct::ChatCompletionMessageCustomToolCall {
                                id: block.id,
                                custom: oct::ChatCompletionMessageCustomToolCallPayload {
                                    input: serde_json::to_string(&block.input)
                                        .unwrap_or_else(|_| "{}".to_string()),
                                    name: block.name,
                                },
                                type_: oct::ChatCompletionMessageCustomToolCallType::Custom,
                            });
                        }
                        crate::claude::create_message::types::BetaContentBlock::McpToolResult(block) => {
                            let output_text = match block.content {
                                Some(cct::BetaMcpToolResultBlockParamContent::Text(text)) => text,
                                Some(cct::BetaMcpToolResultBlockParamContent::Blocks(parts)) => {
                                    parts.into_iter().map(|part| part.text).collect::<Vec<_>>().join("\n")
                                }
                                None => String::new(),
                            };
                            if !output_text.is_empty() {
                                content_parts.push(output_text);
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::Compaction(block) => {
                            if let Some(content) = block.content
                                && !content.is_empty() {
                                    content_parts.push(content);
                                }
                        }
                        crate::claude::create_message::types::BetaContentBlock::ContainerUpload(block) => {
                            content_parts.push(format!("container_upload:{}", block.file_id));
                        }
                        crate::claude::create_message::types::BetaContentBlock::WebSearchToolResult(block) => {
                            let text = match block.content {
                                BetaWebSearchToolResultBlockContent::Results(results) => results
                                    .into_iter()
                                    .map(|item| format!("{}\n{}", item.title, item.url))
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                BetaWebSearchToolResultBlockContent::Error(err) => {
                                    format!("web_search_error:{:?}", err.error_code)
                                }
                            };
                            if !text.is_empty() {
                                content_parts.push(text);
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::WebFetchToolResult(block) => {
                            let text = match block.content {
                                BetaWebFetchToolResultBlockContent::Result(result) => result.url,
                                BetaWebFetchToolResultBlockContent::Error(err) => {
                                    format!("web_fetch_error:{:?}", err.error_code)
                                }
                            };
                            if !text.is_empty() {
                                content_parts.push(text);
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::CodeExecutionToolResult(block) => {
                            let text = match block.content {
                                cct::BetaCodeExecutionToolResultBlockParamContent::Result(result) => {
                                    stdout_stderr_text(result.stdout, result.stderr)
                                }
                                cct::BetaCodeExecutionToolResultBlockParamContent::Error(err) => {
                                    format!("code_execution_error:{:?}", err.error_code)
                                }
                            };
                            if !text.is_empty() {
                                content_parts.push(text);
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::BashCodeExecutionToolResult(block) => {
                            let text = match block.content {
                                cct::BetaBashCodeExecutionToolResultBlockParamContent::Result(result) => {
                                    stdout_stderr_text(result.stdout, result.stderr)
                                }
                                cct::BetaBashCodeExecutionToolResultBlockParamContent::Error(err) => {
                                    format!("bash_code_execution_error:{:?}", err.error_code)
                                }
                            };
                            if !text.is_empty() {
                                content_parts.push(text);
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::TextEditorCodeExecutionToolResult(block) => {
                            let text = match block.content {
                                cct::BetaTextEditorCodeExecutionToolResultBlockParamContent::View(view) => {
                                    view.content
                                }
                                cct::BetaTextEditorCodeExecutionToolResultBlockParamContent::Create(create) => {
                                    format!("file_updated:{}", create.is_file_update)
                                }
                                cct::BetaTextEditorCodeExecutionToolResultBlockParamContent::StrReplace(replace) => {
                                    replace.lines.unwrap_or_default().join("\n")
                                }
                                cct::BetaTextEditorCodeExecutionToolResultBlockParamContent::Error(err) => {
                                    err.error_message.unwrap_or_else(|| {
                                        format!("text_editor_code_execution_error:{:?}", err.error_code)
                                    })
                                }
                            };
                            if !text.is_empty() {
                                content_parts.push(text);
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::ToolSearchToolResult(block) => {
                            let text = match block.content {
                                cct::BetaToolSearchToolResultBlockParamContent::Result(result) => result
                                    .tool_references
                                    .into_iter()
                                    .map(|reference| reference.tool_name)
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                cct::BetaToolSearchToolResultBlockParamContent::Error(err) => {
                                    format!("tool_search_error:{:?}", err.error_code)
                                }
                            };
                            if !text.is_empty() {
                                content_parts.push(text);
                            }
                        }
                        crate::claude::create_message::types::BetaContentBlock::Thinking(_)
                        | crate::claude::create_message::types::BetaContentBlock::RedactedThinking(_) => {}
                    }
                }

                if matches!(stop_reason, Some(BetaStopReason::Refusal)) && content_parts.is_empty()
                {
                    refusal_parts.push("refusal".to_string());
                }

                let function_call = function_tool_calls
                    .first()
                    .map(|call| call.function.clone());

                let mut tool_calls = function_tool_calls
                    .into_iter()
                    .map(oct::ChatCompletionMessageToolCall::Function)
                    .collect::<Vec<_>>();
                tool_calls.extend(
                    custom_tool_calls
                        .into_iter()
                        .map(oct::ChatCompletionMessageToolCall::Custom),
                );

                let finish_reason = match stop_reason.as_ref() {
                    Some(BetaStopReason::MaxTokens)
                    | Some(BetaStopReason::ModelContextWindowExceeded) => {
                        oct::ChatCompletionFinishReason::Length
                    }
                    Some(BetaStopReason::Refusal) => oct::ChatCompletionFinishReason::ContentFilter,
                    Some(BetaStopReason::ToolUse) => oct::ChatCompletionFinishReason::ToolCalls,
                    _ if !tool_calls.is_empty() => oct::ChatCompletionFinishReason::ToolCalls,
                    _ => oct::ChatCompletionFinishReason::Stop,
                };

                let prompt_tokens = usage
                    .input_tokens
                    .saturating_add(usage.cache_creation_input_tokens)
                    .saturating_add(usage.cache_read_input_tokens);
                let total_tokens = prompt_tokens.saturating_add(usage.output_tokens);

                OpenAiChatCompletionsResponse::Success {
                    stats_code,
                    headers: OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: oct::ChatCompletion {
                        id: body.id,
                        choices: vec![oct::ChatCompletionChoice {
                            finish_reason,
                            index: 0,
                            logprobs: None,
                            message: oct::ChatCompletionMessage {
                                content: if content_parts.is_empty() {
                                    None
                                } else {
                                    Some(content_parts.join("\n"))
                                },
                                reasoning_content: None,
                                reasoning_details: None,
                                refusal: if refusal_parts.is_empty() {
                                    None
                                } else {
                                    Some(refusal_parts.join("\n"))
                                },
                                role: oct::ChatCompletionAssistantRole::Assistant,
                                annotations: if annotations.is_empty() {
                                    None
                                } else {
                                    Some(annotations)
                                },
                                audio: None,
                                function_call,
                                tool_calls: if tool_calls.is_empty() {
                                    None
                                } else {
                                    Some(tool_calls)
                                },
                            },
                        }],
                        created: 0,
                        model: claude_model_to_string(&body.model),
                        object: oct::ChatCompletionObject::ChatCompletion,
                        service_tier,
                        system_fingerprint: None,
                        usage: Some(oct::CompletionUsage {
                            completion_tokens: usage.output_tokens,
                            prompt_tokens,
                            total_tokens,
                            completion_tokens_details: Some(oct::CompletionTokensDetails {
                                accepted_prediction_tokens: None,
                                audio_tokens: None,
                                reasoning_tokens: None,
                                rejected_prediction_tokens: None,
                            }),
                            prompt_tokens_details: Some(oct::PromptTokensDetails {
                                audio_tokens: None,
                                cached_tokens: Some(usage.cache_read_input_tokens),
                            }),
                        }),
                    },
                }
            }
            ClaudeCreateMessageResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiChatCompletionsResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_claude(stats_code, body),
            },
        })
    }
}
