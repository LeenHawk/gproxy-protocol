use std::collections::BTreeMap;

use crate::openai::count_tokens::types as ot;
use crate::openai::create_chat_completions::response::OpenAiChatCompletionsResponse;
use crate::openai::create_chat_completions::types as ct;
use crate::openai::create_response::response::{OpenAiCreateResponseResponse, ResponseBody};
use crate::openai::create_response::types as rt;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::utils::TransformError;

fn reasoning_item_from_chat_message(
    fallback_id: String,
    reasoning_content: Option<String>,
    reasoning_details: Option<Vec<ct::ChatCompletionReasoningDetail>>,
) -> Vec<rt::ResponseOutputItem> {
    let mut items = Vec::new();
    let reasoning_content_signature = reasoning_details
        .as_ref()
        .and_then(|details| details.iter().find_map(chat_reasoning_detail_signature));

    if let Some(reasoning_content) = reasoning_content.filter(|text| !text.is_empty()) {
        items.push(reasoning_output_item(
            Some(fallback_id.clone()),
            vec![summary_text_part(reasoning_content.clone())],
            Some(vec![reasoning_text_part(reasoning_content)]),
            None,
            reasoning_content_signature,
        ));
    }

    if let Some(reasoning_details) = reasoning_details {
        let base_ordinal = items.len();
        items.extend(reasoning_details.into_iter().enumerate().filter_map(
            |(detail_index, detail)| {
                chat_reasoning_detail_to_response_output_item(
                    fallback_id.as_str(),
                    base_ordinal + detail_index,
                    detail,
                )
            },
        ));
    }

    items
}

fn chat_reasoning_detail_signature(detail: &ct::ChatCompletionReasoningDetail) -> Option<String> {
    detail
        .signature
        .clone()
        .filter(|signature| !signature.is_empty())
        .or_else(|| detail.id.clone().filter(|id| !id.is_empty()))
}

fn reasoning_text_part(text: String) -> ot::ResponseReasoningTextContent {
    ot::ResponseReasoningTextContent {
        text,
        type_: ot::ResponseReasoningTextContentType::ReasoningText,
    }
}

fn summary_text_part(text: String) -> ot::ResponseSummaryTextContent {
    ot::ResponseSummaryTextContent {
        text,
        type_: ot::ResponseSummaryTextContentType::SummaryText,
    }
}

fn reasoning_output_item(
    id: Option<String>,
    summary: Vec<ot::ResponseSummaryTextContent>,
    content: Option<Vec<ot::ResponseReasoningTextContent>>,
    encrypted_content: Option<String>,
    signature: Option<String>,
) -> rt::ResponseOutputItem {
    rt::ResponseOutputItem::ReasoningItem(ot::ResponseReasoningItem {
        id,
        summary,
        type_: ot::ResponseReasoningItemType::Reasoning,
        content,
        encrypted_content,
        status: Some(ot::ResponseItemStatus::Completed),
        signature,
    })
}

fn chat_reasoning_detail_to_response_output_item(
    fallback_id: &str,
    ordinal: usize,
    detail: ct::ChatCompletionReasoningDetail,
) -> Option<rt::ResponseOutputItem> {
    let id = detail
        .id
        .or_else(|| Some(format!("{fallback_id}_{ordinal}")));
    let signature = detail.signature;

    match detail.type_ {
        ct::ChatCompletionReasoningDetailType::ReasoningEncrypted => detail
            .data
            .map(|data| reasoning_output_item(id, Vec::new(), None, Some(data), signature)),
        ct::ChatCompletionReasoningDetailType::ReasoningSummary => detail.text.map(|text| {
            reasoning_output_item(id, vec![summary_text_part(text)], None, None, signature)
        }),
        ct::ChatCompletionReasoningDetailType::ReasoningText => detail.text.map(|text| {
            reasoning_output_item(
                id,
                Vec::new(),
                Some(vec![reasoning_text_part(text)]),
                None,
                signature,
            )
        }),
    }
}
impl TryFrom<OpenAiChatCompletionsResponse> for OpenAiCreateResponseResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiChatCompletionsResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiChatCompletionsResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let choice = body.choices.into_iter().next();
                let mut output = Vec::new();
                let mut output_text_parts = Vec::new();
                let mut tool_call_count = 0usize;

                let mut status = Some(rt::ResponseStatus::Completed);
                let mut incomplete_details = None;

                if let Some(choice) = choice {
                    match choice.finish_reason {
                        ct::ChatCompletionFinishReason::Length => {
                            status = Some(rt::ResponseStatus::Incomplete);
                            incomplete_details = Some(rt::ResponseIncompleteDetails {
                                reason: Some(rt::ResponseIncompleteReason::MaxOutputTokens),
                            });
                        }
                        ct::ChatCompletionFinishReason::ContentFilter => {
                            status = Some(rt::ResponseStatus::Incomplete);
                            incomplete_details = Some(rt::ResponseIncompleteDetails {
                                reason: Some(rt::ResponseIncompleteReason::ContentFilter),
                            });
                        }
                        ct::ChatCompletionFinishReason::Stop
                        | ct::ChatCompletionFinishReason::ToolCalls
                        | ct::ChatCompletionFinishReason::FunctionCall => {}
                    }

                    output.extend(reasoning_item_from_chat_message(
                        format!("{}_reasoning_0", body.id),
                        choice.message.reasoning_content.clone(),
                        choice.message.reasoning_details.clone(),
                    ));

                    let mut message_content = Vec::new();
                    if let Some(content) = choice.message.content
                        && !content.is_empty()
                    {
                        output_text_parts.push(content.clone());
                        message_content.push(ot::ResponseOutputContent::Text(
                            ot::ResponseOutputText {
                                annotations: Vec::new(),
                                logprobs: None,
                                text: content,
                                type_: ot::ResponseOutputTextType::OutputText,
                            },
                        ));
                    }
                    if let Some(refusal) = choice.message.refusal
                        && !refusal.is_empty()
                    {
                        message_content.push(ot::ResponseOutputContent::Refusal(
                            ot::ResponseOutputRefusal {
                                refusal,
                                type_: ot::ResponseOutputRefusalType::Refusal,
                            },
                        ));
                    }

                    if !message_content.is_empty() {
                        output.push(rt::ResponseOutputItem::Message(ot::ResponseOutputMessage {
                            id: format!("{}_message_0", body.id),
                            content: message_content,
                            role: ot::ResponseOutputMessageRole::Assistant,
                            phase: Some(ot::ResponseMessagePhase::FinalAnswer),
                            status: Some(ot::ResponseItemStatus::Completed),
                            type_: Some(ot::ResponseOutputMessageType::Message),
                        }));
                    }

                    if let Some(function_call) = choice.message.function_call {
                        tool_call_count += 1;
                        output.push(rt::ResponseOutputItem::FunctionToolCall(
                            ot::ResponseFunctionToolCall {
                                arguments: function_call.arguments,
                                call_id: "function_call".to_string(),
                                name: function_call.name,
                                type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                id: Some("function_call".to_string()),
                                status: None,
                            },
                        ));
                    }

                    if let Some(tool_calls) = choice.message.tool_calls {
                        for tool_call in tool_calls {
                            match tool_call {
                                ct::ChatCompletionMessageToolCall::Function(call) => {
                                    tool_call_count += 1;
                                    output.push(rt::ResponseOutputItem::FunctionToolCall(
                                        ot::ResponseFunctionToolCall {
                                            arguments: call.function.arguments,
                                            call_id: call.id.clone(),
                                            name: call.function.name,
                                            type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                            id: Some(call.id),
                                            status: None,
                                        },
                                    ));
                                }
                                ct::ChatCompletionMessageToolCall::Custom(call) => {
                                    tool_call_count += 1;
                                    output.push(rt::ResponseOutputItem::CustomToolCall(
                                        ot::ResponseCustomToolCall {
                                            call_id: call.id.clone(),
                                            input: call.custom.input,
                                            name: call.custom.name,
                                            type_: ot::ResponseCustomToolCallType::CustomToolCall,
                                            id: Some(call.id),
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }

                OpenAiCreateResponseResponse::Success {
                    stats_code,
                    headers: OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ResponseBody {
                        id: body.id,
                        created_at: body.created,
                        error: None,
                        incomplete_details,
                        instructions: Some(ot::ResponseInput::Text(String::new())),
                        metadata: BTreeMap::new(),
                        model: body.model,
                        object: rt::ResponseObject::Response,
                        output,
                        parallel_tool_calls: tool_call_count > 1,
                        temperature: 1.0,
                        tool_choice: if tool_call_count > 0 {
                            ot::ResponseToolChoice::Options(ot::ResponseToolChoiceOptions::Required)
                        } else {
                            ot::ResponseToolChoice::Options(ot::ResponseToolChoiceOptions::Auto)
                        },
                        tools: Vec::new(),
                        top_p: 1.0,
                        background: None,
                        completed_at: None,
                        conversation: None,
                        max_output_tokens: None,
                        max_tool_calls: None,
                        output_text: if output_text_parts.is_empty() {
                            None
                        } else {
                            Some(output_text_parts.join("\n"))
                        },
                        previous_response_id: None,
                        prompt: None,
                        prompt_cache_key: None,
                        prompt_cache_retention: None,
                        reasoning: None,
                        safety_identifier: None,
                        service_tier: body.service_tier.map(|tier| match tier {
                            ct::ChatCompletionServiceTier::Auto => rt::ResponseServiceTier::Auto,
                            ct::ChatCompletionServiceTier::Default => {
                                rt::ResponseServiceTier::Default
                            }
                            ct::ChatCompletionServiceTier::Flex => rt::ResponseServiceTier::Flex,
                            ct::ChatCompletionServiceTier::Scale => rt::ResponseServiceTier::Scale,
                            ct::ChatCompletionServiceTier::Priority => {
                                rt::ResponseServiceTier::Priority
                            }
                        }),
                        status,
                        text: None,
                        top_logprobs: None,
                        truncation: None,
                        usage: body.usage.map(|usage| {
                            let cached_tokens = usage
                                .prompt_tokens_details
                                .as_ref()
                                .and_then(|details| details.cached_tokens)
                                .unwrap_or(0);
                            let reasoning_tokens = usage
                                .completion_tokens_details
                                .as_ref()
                                .and_then(|details| details.reasoning_tokens)
                                .unwrap_or(0);
                            rt::ResponseUsage {
                                input_tokens: usage.prompt_tokens,
                                input_tokens_details: rt::ResponseInputTokensDetails {
                                    cached_tokens,
                                },
                                output_tokens: usage.completion_tokens,
                                output_tokens_details: rt::ResponseOutputTokensDetails {
                                    reasoning_tokens,
                                },
                                total_tokens: usage.total_tokens,
                            }
                        }),
                        user: None,
                    },
                }
            }
            OpenAiChatCompletionsResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCreateResponseResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_response_reasoning_details_preserve_response_reasoning_items() {
        let response = OpenAiChatCompletionsResponse::Success {
            stats_code: http::StatusCode::OK,
            headers: OpenAiResponseHeaders::default(),
            body: ct::ChatCompletion {
                id: "chatcmpl_1".to_string(),
                choices: vec![ct::ChatCompletionChoice {
                    finish_reason: ct::ChatCompletionFinishReason::Stop,
                    index: 0,
                    logprobs: None,
                    message: ct::ChatCompletionMessage {
                        content: Some("done".to_string()),
                        reasoning_content: None,
                        reasoning_details: Some(vec![
                            ct::ChatCompletionReasoningDetail {
                                type_: ct::ChatCompletionReasoningDetailType::ReasoningEncrypted,
                                id: Some("enc_1".to_string()),
                                data: Some("ciphertext".to_string()),
                                text: None,
                                signature: Some("sig_enc".to_string()),
                                index: Some(0),
                            },
                            ct::ChatCompletionReasoningDetail {
                                type_: ct::ChatCompletionReasoningDetailType::ReasoningText,
                                id: Some("txt_1".to_string()),
                                data: None,
                                text: Some("detail text".to_string()),
                                signature: Some("sig_text".to_string()),
                                index: Some(1),
                            },
                        ]),
                        refusal: None,
                        role: ct::ChatCompletionAssistantRole::Assistant,
                        annotations: None,
                        audio: None,
                        function_call: None,
                        tool_calls: None,
                    },
                }],
                created: 1,
                model: "gpt-5".to_string(),
                object: ct::ChatCompletionObject::ChatCompletion,
                service_tier: None,
                system_fingerprint: None,
                usage: None,
            },
        };

        let converted = OpenAiCreateResponseResponse::try_from(response).unwrap();
        let body = match converted {
            OpenAiCreateResponseResponse::Success { body, .. } => body,
            _ => panic!("expected success response"),
        };
        let reasoning = body
            .output
            .into_iter()
            .filter_map(|item| match item {
                rt::ResponseOutputItem::ReasoningItem(item) => Some(item),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(reasoning.len(), 2);
        assert!(reasoning.iter().any(|item| {
            item.id.as_deref() == Some("enc_1")
                && item.encrypted_content.as_deref() == Some("ciphertext")
                && item.signature.as_deref() == Some("sig_enc")
        }));
        assert!(reasoning.iter().any(|item| {
            item.id.as_deref() == Some("txt_1")
                && item
                    .content
                    .as_ref()
                    .is_some_and(|parts| parts.iter().any(|part| part.text == "detail text"))
                && item.signature.as_deref() == Some("sig_text")
        }));
    }
}
