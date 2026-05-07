use crate::openai::count_tokens::types as ot;
use crate::openai::create_chat_completions::response::OpenAiChatCompletionsResponse;
use crate::openai::create_chat_completions::types as ct;
use crate::openai::create_response::response::OpenAiCreateResponseResponse;
use crate::openai::create_response::types as rt;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::utils::TransformError;

fn reasoning_item_to_visible_text(reasoning: &ot::ResponseReasoningItem) -> String {
    if let Some(content) = reasoning.content.as_ref() {
        let text = content
            .iter()
            .map(|part| part.text.as_str())
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        if !text.is_empty() {
            return text;
        }
    }

    reasoning
        .summary
        .iter()
        .map(|part| part.text.as_str())
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn chat_reasoning_detail(
    type_: ct::ChatCompletionReasoningDetailType,
    id: Option<String>,
    data: Option<String>,
    text: Option<String>,
    signature: Option<String>,
    index: Option<u64>,
) -> ct::ChatCompletionReasoningDetail {
    ct::ChatCompletionReasoningDetail {
        type_,
        id,
        data,
        text,
        signature,
        index,
    }
}

fn reasoning_item_to_chat_details(
    reasoning: &ot::ResponseReasoningItem,
) -> Vec<ct::ChatCompletionReasoningDetail> {
    let mut details = Vec::new();
    let id = reasoning.id.clone();
    let signature = reasoning.signature.clone();
    let mut index = 0_u64;

    if let Some(encrypted_content) = reasoning
        .encrypted_content
        .as_ref()
        .filter(|text| !text.is_empty())
    {
        details.push(chat_reasoning_detail(
            ct::ChatCompletionReasoningDetailType::ReasoningEncrypted,
            id.clone(),
            Some(encrypted_content.clone()),
            None,
            signature.clone(),
            Some(index),
        ));
        index += 1;
    }

    if let Some(content) = reasoning.content.as_ref() {
        for part in content.iter().filter(|part| !part.text.is_empty()) {
            details.push(chat_reasoning_detail(
                ct::ChatCompletionReasoningDetailType::ReasoningText,
                id.clone(),
                None,
                Some(part.text.clone()),
                signature.clone(),
                Some(index),
            ));
            index += 1;
        }
    }

    for part in reasoning
        .summary
        .iter()
        .filter(|part| !part.text.is_empty())
    {
        details.push(chat_reasoning_detail(
            ct::ChatCompletionReasoningDetailType::ReasoningSummary,
            id.clone(),
            None,
            Some(part.text.clone()),
            signature.clone(),
            Some(index),
        ));
        index += 1;
    }

    details
}

impl TryFrom<OpenAiCreateResponseResponse> for OpenAiChatCompletionsResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiCreateResponseResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiCreateResponseResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let mut content_parts = Vec::new();
                let mut refusal_parts = Vec::new();
                let mut reasoning_content_parts = Vec::new();
                let mut reasoning_details = Vec::new();
                let mut function_calls = Vec::new();
                let mut custom_calls = Vec::new();

                for item in &body.output {
                    match item {
                        rt::ResponseOutputItem::Message(message) => {
                            for content in &message.content {
                                match content {
                                    ot::ResponseOutputContent::Text(text) => {
                                        if !text.text.is_empty() {
                                            content_parts.push(text.text.clone());
                                        }
                                    }
                                    ot::ResponseOutputContent::Refusal(refusal) => {
                                        if !refusal.refusal.is_empty() {
                                            refusal_parts.push(refusal.refusal.clone());
                                        }
                                    }
                                }
                            }
                        }
                        rt::ResponseOutputItem::FunctionToolCall(call) => {
                            function_calls.push(ct::ChatCompletionMessageFunctionToolCall {
                                id: call.call_id.clone(),
                                function: ct::ChatCompletionFunctionCall {
                                    arguments: call.arguments.clone(),
                                    name: call.name.clone(),
                                },
                                type_: ct::ChatCompletionMessageFunctionToolCallType::Function,
                            });
                        }
                        rt::ResponseOutputItem::CustomToolCall(call) => {
                            custom_calls.push(ct::ChatCompletionMessageCustomToolCall {
                                id: call.call_id.clone(),
                                custom: ct::ChatCompletionMessageCustomToolCallPayload {
                                    input: call.input.clone(),
                                    name: call.name.clone(),
                                },
                                type_: ct::ChatCompletionMessageCustomToolCallType::Custom,
                            });
                        }
                        rt::ResponseOutputItem::ReasoningItem(reasoning) => {
                            let text = reasoning_item_to_visible_text(reasoning);
                            if !text.is_empty() {
                                reasoning_content_parts.push(text);
                            }
                            reasoning_details.extend(reasoning_item_to_chat_details(reasoning));
                        }
                        _ => {}
                    }
                }

                let function_call = function_calls.first().map(|call| call.function.clone());

                let mut tool_calls = function_calls
                    .into_iter()
                    .map(ct::ChatCompletionMessageToolCall::Function)
                    .collect::<Vec<_>>();
                tool_calls.extend(
                    custom_calls
                        .into_iter()
                        .map(ct::ChatCompletionMessageToolCall::Custom),
                );

                let finish_reason = if matches!(
                    body.incomplete_details
                        .as_ref()
                        .and_then(|d| d.reason.as_ref()),
                    Some(rt::ResponseIncompleteReason::MaxOutputTokens)
                ) {
                    ct::ChatCompletionFinishReason::Length
                } else if matches!(
                    body.incomplete_details
                        .as_ref()
                        .and_then(|d| d.reason.as_ref()),
                    Some(rt::ResponseIncompleteReason::ContentFilter)
                ) {
                    ct::ChatCompletionFinishReason::ContentFilter
                } else if !tool_calls.is_empty() {
                    ct::ChatCompletionFinishReason::ToolCalls
                } else {
                    ct::ChatCompletionFinishReason::Stop
                };

                OpenAiChatCompletionsResponse::Success {
                    stats_code,
                    headers: OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ct::ChatCompletion {
                        id: body.id,
                        choices: vec![ct::ChatCompletionChoice {
                            finish_reason,
                            index: 0,
                            logprobs: None,
                            message: ct::ChatCompletionMessage {
                                content: if content_parts.is_empty() {
                                    None
                                } else {
                                    Some(content_parts.join("\n"))
                                },
                                reasoning_content: if reasoning_content_parts.is_empty() {
                                    None
                                } else {
                                    Some(reasoning_content_parts.join("\n"))
                                },
                                reasoning_details: if reasoning_details.is_empty() {
                                    None
                                } else {
                                    Some(reasoning_details)
                                },
                                refusal: if refusal_parts.is_empty() {
                                    None
                                } else {
                                    Some(refusal_parts.join("\n"))
                                },
                                role: ct::ChatCompletionAssistantRole::Assistant,
                                annotations: None,
                                audio: None,
                                function_call,
                                tool_calls: if tool_calls.is_empty() {
                                    None
                                } else {
                                    Some(tool_calls)
                                },
                            },
                        }],
                        created: body.created_at,
                        model: body.model,
                        object: ct::ChatCompletionObject::ChatCompletion,
                        service_tier: body.service_tier.map(|tier| match tier {
                            rt::ResponseServiceTier::Auto => ct::ChatCompletionServiceTier::Auto,
                            rt::ResponseServiceTier::Default => {
                                ct::ChatCompletionServiceTier::Default
                            }
                            rt::ResponseServiceTier::Flex => ct::ChatCompletionServiceTier::Flex,
                            rt::ResponseServiceTier::Scale => ct::ChatCompletionServiceTier::Scale,
                            rt::ResponseServiceTier::Priority => {
                                ct::ChatCompletionServiceTier::Priority
                            }
                        }),
                        system_fingerprint: None,
                        usage: body.usage.map(|usage| ct::CompletionUsage {
                            completion_tokens: usage.output_tokens,
                            prompt_tokens: usage.input_tokens,
                            total_tokens: usage.total_tokens,
                            completion_tokens_details: Some(ct::CompletionTokensDetails {
                                accepted_prediction_tokens: None,
                                audio_tokens: None,
                                reasoning_tokens: Some(
                                    usage.output_tokens_details.reasoning_tokens,
                                ),
                                rejected_prediction_tokens: None,
                            }),
                            prompt_tokens_details: Some(ct::PromptTokensDetails {
                                audio_tokens: None,
                                cached_tokens: Some(usage.input_tokens_details.cached_tokens),
                            }),
                        }),
                    },
                }
            }
            OpenAiCreateResponseResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiChatCompletionsResponse::Error {
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
    use crate::openai::create_response::response::ResponseBody;
    use std::collections::BTreeMap;

    #[test]
    fn response_reasoning_items_preserve_chat_response_reasoning_details() {
        let response = OpenAiCreateResponseResponse::Success {
            stats_code: http::StatusCode::OK,
            headers: OpenAiResponseHeaders::default(),
            body: ResponseBody {
                id: "resp_1".to_string(),
                created_at: 1,
                error: None,
                incomplete_details: None,
                instructions: None,
                metadata: BTreeMap::new(),
                model: "gpt-5".to_string(),
                object: rt::ResponseObject::Response,
                output: vec![rt::ResponseOutputItem::ReasoningItem(
                    ot::ResponseReasoningItem {
                        id: Some("rs_1".to_string()),
                        summary: vec![ot::ResponseSummaryTextContent {
                            text: "summary text".to_string(),
                            type_: ot::ResponseSummaryTextContentType::SummaryText,
                        }],
                        type_: ot::ResponseReasoningItemType::Reasoning,
                        content: Some(vec![ot::ResponseReasoningTextContent {
                            text: "visible text".to_string(),
                            type_: ot::ResponseReasoningTextContentType::ReasoningText,
                        }]),
                        encrypted_content: Some("ciphertext".to_string()),
                        status: Some(ot::ResponseItemStatus::Completed),
                        signature: Some("sig_rs".to_string()),
                    },
                )],
                parallel_tool_calls: false,
                temperature: 1.0,
                tool_choice: ot::ResponseToolChoice::Options(ot::ResponseToolChoiceOptions::Auto),
                tools: Vec::new(),
                top_p: 1.0,
                background: None,
                completed_at: None,
                conversation: None,
                max_output_tokens: None,
                max_tool_calls: None,
                output_text: None,
                previous_response_id: None,
                prompt: None,
                prompt_cache_key: None,
                prompt_cache_retention: None,
                reasoning: None,
                safety_identifier: None,
                service_tier: None,
                status: Some(rt::ResponseStatus::Completed),
                text: None,
                top_logprobs: None,
                truncation: None,
                usage: None,
                user: None,
            },
        };

        let converted = OpenAiChatCompletionsResponse::try_from(response).unwrap();
        let body = match converted {
            OpenAiChatCompletionsResponse::Success { body, .. } => body,
            _ => panic!("expected success response"),
        };
        let message = &body.choices[0].message;

        assert_eq!(message.reasoning_content.as_deref(), Some("visible text"));
        let details = message
            .reasoning_details
            .as_ref()
            .expect("reasoning details");
        assert!(details.iter().any(|detail| {
            matches!(
                detail.type_,
                ct::ChatCompletionReasoningDetailType::ReasoningEncrypted
            ) && detail.id.as_deref() == Some("rs_1")
                && detail.data.as_deref() == Some("ciphertext")
                && detail.signature.as_deref() == Some("sig_rs")
        }));
        assert!(details.iter().any(|detail| {
            matches!(
                detail.type_,
                ct::ChatCompletionReasoningDetailType::ReasoningText
            ) && detail.text.as_deref() == Some("visible text")
        }));
        assert!(details.iter().any(|detail| {
            matches!(
                detail.type_,
                ct::ChatCompletionReasoningDetailType::ReasoningSummary
            ) && detail.text.as_deref() == Some("summary text")
        }));
    }
}
