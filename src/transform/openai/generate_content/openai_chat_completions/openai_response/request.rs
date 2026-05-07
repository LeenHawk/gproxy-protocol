use crate::openai::count_tokens::types as ot;
use crate::openai::create_chat_completions::request::OpenAiChatCompletionsRequest;
use crate::openai::create_chat_completions::types as ct;
use crate::openai::create_response::request::{
    OpenAiCreateResponseRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::create_response::types::HttpMethod as OpenAiHttpMethod;
use crate::openai::create_response::types::{
    ResponsePromptCacheRetention, ResponseServiceTier, ResponseStreamOptions,
};
use crate::transform::openai::generate_content::openai_chat_completions::utils::{
    chat_reasoning_to_response_reasoning, chat_response_text_config,
    chat_text_content_to_plain_text, chat_text_content_to_response_input_message_content,
    chat_tool_choice_to_response_tool_choice, chat_tools_to_response_tools,
    chat_user_content_to_response_input_message_content, pseudo_reasoning_signature,
};
use crate::transform::utils::TransformError;

fn response_reasoning_text(text: String) -> ot::ResponseReasoningTextContent {
    ot::ResponseReasoningTextContent {
        text,
        type_: ot::ResponseReasoningTextContentType::ReasoningText,
    }
}

fn response_summary_text(text: String) -> ot::ResponseSummaryTextContent {
    ot::ResponseSummaryTextContent {
        text,
        type_: ot::ResponseSummaryTextContentType::SummaryText,
    }
}

fn response_reasoning_item(
    id: Option<String>,
    summary: Vec<ot::ResponseSummaryTextContent>,
    content: Option<Vec<ot::ResponseReasoningTextContent>>,
    encrypted_content: Option<String>,
    signature: Option<String>,
) -> ot::ResponseInputItem {
    ot::ResponseInputItem::ReasoningItem(ot::ResponseReasoningItem {
        id,
        summary,
        type_: ot::ResponseReasoningItemType::Reasoning,
        content,
        encrypted_content,
        status: None,
        signature,
    })
}

fn chat_reasoning_detail_to_response_item(
    message_index: usize,
    ordinal: usize,
    detail: ct::ChatCompletionReasoningDetail,
) -> Option<ot::ResponseInputItem> {
    let fallback_id = pseudo_reasoning_signature(message_index, ordinal);
    let id = detail.id.or(Some(fallback_id));
    let signature = detail.signature;

    match detail.type_ {
        ct::ChatCompletionReasoningDetailType::ReasoningEncrypted => detail
            .data
            .map(|data| response_reasoning_item(id, Vec::new(), None, Some(data), signature)),
        ct::ChatCompletionReasoningDetailType::ReasoningSummary => detail.text.map(|text| {
            response_reasoning_item(id, vec![response_summary_text(text)], None, None, signature)
        }),
        ct::ChatCompletionReasoningDetailType::ReasoningText => detail.text.map(|text| {
            response_reasoning_item(
                id,
                Vec::new(),
                Some(vec![response_reasoning_text(text)]),
                None,
                signature,
            )
        }),
    }
}

fn chat_reasoning_detail_signature(detail: &ct::ChatCompletionReasoningDetail) -> Option<String> {
    detail
        .signature
        .clone()
        .filter(|signature| !signature.is_empty())
        .or_else(|| detail.id.clone().filter(|id| !id.is_empty()))
}

impl TryFrom<OpenAiChatCompletionsRequest> for OpenAiCreateResponseRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiChatCompletionsRequest) -> Result<Self, TransformError> {
        let crate::openai::create_chat_completions::request::RequestBody {
            messages,
            model,
            function_call,
            functions,
            max_completion_tokens,
            max_tokens,
            metadata,
            parallel_tool_calls,
            prompt_cache_key,
            prompt_cache_retention,
            reasoning_effort,
            response_format,
            service_tier,
            store,
            stream,
            stream_options,
            temperature,
            tool_choice,
            tools,
            top_logprobs,
            top_p,
            user,
            verbosity,
            web_search_options,
            ..
        } = value.body;

        let mut input_items = Vec::new();
        let mut instructions_parts: Vec<String> = Vec::new();

        for (index, message) in messages.into_iter().enumerate() {
            match message {
                ct::ChatCompletionMessageParam::Developer(message) => {
                    input_items.push(ot::ResponseInputItem::Message(ot::ResponseInputMessage {
                        content: chat_text_content_to_response_input_message_content(
                            message.content,
                        ),
                        role: ot::ResponseInputMessageRole::Developer,
                        phase: None,
                        status: None,
                        type_: Some(ot::ResponseInputMessageType::Message),
                    }));
                }
                ct::ChatCompletionMessageParam::System(message) => {
                    let text = chat_text_content_to_plain_text(&message.content);
                    if !text.is_empty() {
                        instructions_parts.push(text);
                    }
                }
                ct::ChatCompletionMessageParam::User(message) => {
                    input_items.push(ot::ResponseInputItem::Message(ot::ResponseInputMessage {
                        content: chat_user_content_to_response_input_message_content(
                            message.content,
                        ),
                        role: ot::ResponseInputMessageRole::User,
                        phase: None,
                        status: None,
                        type_: Some(ot::ResponseInputMessageType::Message),
                    }));
                }
                ct::ChatCompletionMessageParam::Assistant(message) => {
                    let ct::ChatCompletionAssistantMessageParam {
                        content,
                        reasoning_content,
                        reasoning_details,
                        function_call,
                        refusal,
                        tool_calls,
                        ..
                    } = message;
                    let mut output_content = Vec::new();

                    if let Some(content) = content {
                        match content {
                            ct::ChatCompletionAssistantContent::Text(text) => {
                                if !text.is_empty() {
                                    output_content.push(ot::ResponseOutputContent::Text(
                                        ot::ResponseOutputText {
                                            annotations: Vec::new(),
                                            logprobs: None,
                                            text,
                                            type_: ot::ResponseOutputTextType::OutputText,
                                        },
                                    ));
                                }
                            }
                            ct::ChatCompletionAssistantContent::Parts(parts) => {
                                for part in parts {
                                    match part {
                                        ct::ChatCompletionAssistantContentPart::Text(part) => {
                                            if !part.text.is_empty() {
                                                output_content
                                                    .push(ot::ResponseOutputContent::Text(
                                                    ot::ResponseOutputText {
                                                        annotations: Vec::new(),
                                                        logprobs: None,
                                                        text: part.text,
                                                        type_:
                                                            ot::ResponseOutputTextType::OutputText,
                                                    },
                                                ));
                                            }
                                        }
                                        ct::ChatCompletionAssistantContentPart::Refusal(part) => {
                                            if !part.refusal.is_empty() {
                                                output_content
                                                    .push(ot::ResponseOutputContent::Refusal(
                                                    ot::ResponseOutputRefusal {
                                                        refusal: part.refusal,
                                                        type_:
                                                            ot::ResponseOutputRefusalType::Refusal,
                                                    },
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(refusal) = refusal
                        && !refusal.is_empty()
                    {
                        output_content.push(ot::ResponseOutputContent::Refusal(
                            ot::ResponseOutputRefusal {
                                refusal,
                                type_: ot::ResponseOutputRefusalType::Refusal,
                            },
                        ));
                    }

                    if !output_content.is_empty() {
                        input_items.push(ot::ResponseInputItem::OutputMessage(
                            ot::ResponseOutputMessage {
                                id: format!("msg_{index}"),
                                content: output_content,
                                role: ot::ResponseOutputMessageRole::Assistant,
                                phase: Some(ot::ResponseMessagePhase::FinalAnswer),
                                status: Some(ot::ResponseItemStatus::Completed),
                                type_: Some(ot::ResponseOutputMessageType::Message),
                            },
                        ));
                    }

                    let has_reasoning_content = reasoning_content
                        .as_ref()
                        .is_some_and(|text| !text.is_empty());
                    let reasoning_content_signature =
                        reasoning_details.as_ref().and_then(|details| {
                            details.iter().find_map(chat_reasoning_detail_signature)
                        });
                    if let Some(reasoning_content) = reasoning_content
                        && !reasoning_content.is_empty()
                    {
                        input_items.push(response_reasoning_item(
                            Some(pseudo_reasoning_signature(index, 0)),
                            vec![response_summary_text(reasoning_content.clone())],
                            Some(vec![response_reasoning_text(reasoning_content)]),
                            None,
                            reasoning_content_signature,
                        ));
                    }

                    if let Some(reasoning_details) = reasoning_details {
                        let base_ordinal = if has_reasoning_content { 1 } else { 0 };
                        input_items.extend(reasoning_details.into_iter().enumerate().filter_map(
                            |(detail_index, detail)| {
                                chat_reasoning_detail_to_response_item(
                                    index,
                                    base_ordinal + detail_index,
                                    detail,
                                )
                            },
                        ));
                    }

                    if let Some(function_call) = function_call {
                        input_items.push(ot::ResponseInputItem::FunctionToolCall(
                            ot::ResponseFunctionToolCall {
                                arguments: function_call.arguments,
                                call_id: format!("function_call_{index}"),
                                name: function_call.name,
                                type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                id: None,
                                status: None,
                            },
                        ));
                    }

                    if let Some(tool_calls) = tool_calls {
                        for call in tool_calls {
                            match call {
                                ct::ChatCompletionMessageToolCall::Function(call) => {
                                    input_items.push(ot::ResponseInputItem::FunctionToolCall(
                                        ot::ResponseFunctionToolCall {
                                            arguments: call.function.arguments,
                                            call_id: call.id.clone(),
                                            name: call.function.name,
                                            type_: ot::ResponseFunctionToolCallType::FunctionCall,
                                            id: None,
                                            status: None,
                                        },
                                    ));
                                }
                                ct::ChatCompletionMessageToolCall::Custom(call) => {
                                    input_items.push(ot::ResponseInputItem::CustomToolCall(
                                        ot::ResponseCustomToolCall {
                                            call_id: call.id.clone(),
                                            input: call.custom.input,
                                            name: call.custom.name,
                                            type_: ot::ResponseCustomToolCallType::CustomToolCall,
                                            id: None,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }
                ct::ChatCompletionMessageParam::Tool(message) => {
                    input_items.push(ot::ResponseInputItem::FunctionCallOutput(
                        ot::ResponseFunctionCallOutput {
                            call_id: message.tool_call_id,
                            output: ot::ResponseFunctionCallOutputContent::Text(
                                chat_text_content_to_plain_text(&message.content),
                            ),
                            type_: ot::ResponseFunctionCallOutputType::FunctionCallOutput,
                            id: None,
                            status: Some(ot::ResponseItemStatus::Completed),
                        },
                    ));
                }
                ct::ChatCompletionMessageParam::Function(message) => {
                    input_items.push(ot::ResponseInputItem::FunctionCallOutput(
                        ot::ResponseFunctionCallOutput {
                            call_id: message.name,
                            output: ot::ResponseFunctionCallOutputContent::Text(message.content),
                            type_: ot::ResponseFunctionCallOutputType::FunctionCallOutput,
                            id: None,
                            status: Some(ot::ResponseItemStatus::Completed),
                        },
                    ));
                }
            }
        }

        let service_tier = service_tier.map(|tier| match tier {
            ct::ChatCompletionServiceTier::Auto => ResponseServiceTier::Auto,
            ct::ChatCompletionServiceTier::Default => ResponseServiceTier::Default,
            ct::ChatCompletionServiceTier::Flex => ResponseServiceTier::Flex,
            ct::ChatCompletionServiceTier::Scale => ResponseServiceTier::Scale,
            ct::ChatCompletionServiceTier::Priority => ResponseServiceTier::Priority,
        });

        let prompt_cache_retention = prompt_cache_retention.map(|value| match value {
            ct::ChatCompletionPromptCacheRetention::InMemory => {
                ResponsePromptCacheRetention::InMemory
            }
            ct::ChatCompletionPromptCacheRetention::H24 => ResponsePromptCacheRetention::H24,
        });

        let stream_options = stream_options.and_then(|options| {
            options
                .include_obfuscation
                .map(|include_obfuscation| ResponseStreamOptions {
                    include_obfuscation: Some(include_obfuscation),
                })
        });

        Ok(OpenAiCreateResponseRequest {
            method: OpenAiHttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                background: None,
                context_management: None,
                conversation: None,
                include: None,
                input: if input_items.is_empty() {
                    None
                } else {
                    Some(ot::ResponseInput::Items(input_items))
                },
                instructions: if instructions_parts.is_empty() {
                    None
                } else {
                    Some(instructions_parts.join(" "))
                },
                max_output_tokens: max_completion_tokens.or(max_tokens),
                max_tool_calls: None,
                metadata,
                model: Some(model),
                parallel_tool_calls,
                previous_response_id: None,
                prompt: None,
                prompt_cache_key,
                prompt_cache_retention,
                reasoning: chat_reasoning_to_response_reasoning(reasoning_effort),
                safety_identifier: None,
                service_tier,
                store,
                stream,
                stream_options,
                temperature,
                text: chat_response_text_config(response_format, verbosity),
                tool_choice: chat_tool_choice_to_response_tool_choice(tool_choice, function_call),
                tools: chat_tools_to_response_tools(tools, functions, web_search_options),
                top_logprobs,
                top_p,
                truncation: None,
                user,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::create_chat_completions::request as oreq;

    #[test]
    fn chat_assistant_history_uses_msg_prefixed_output_message_id() {
        let request = OpenAiChatCompletionsRequest {
            method: ct::HttpMethod::Post,
            path: oreq::PathParameters::default(),
            query: oreq::QueryParameters::default(),
            headers: oreq::RequestHeaders::default(),
            body: oreq::RequestBody {
                model: "gpt-5".to_string(),
                messages: vec![ct::ChatCompletionMessageParam::Assistant(
                    ct::ChatCompletionAssistantMessageParam {
                        role: ct::ChatCompletionAssistantRole::Assistant,
                        audio: None,
                        content: Some(ct::ChatCompletionAssistantContent::Text(
                            "previous answer".to_string(),
                        )),
                        reasoning_content: None,
                        reasoning_details: None,
                        function_call: None,
                        name: None,
                        refusal: None,
                        tool_calls: None,
                    },
                )],
                ..oreq::RequestBody::default()
            },
        };

        let converted = OpenAiCreateResponseRequest::try_from(request).unwrap();
        let items = match converted.body.input {
            Some(ot::ResponseInput::Items(items)) => items,
            other => panic!("unexpected input: {other:?}"),
        };

        let output_message = items
            .into_iter()
            .find_map(|item| match item {
                ot::ResponseInputItem::OutputMessage(message) => Some(message),
                _ => None,
            })
            .expect("output message");

        assert_eq!(output_message.id, "msg_0");
    }

    #[test]
    fn chat_assistant_history_preserves_reasoning_details() {
        let request = OpenAiChatCompletionsRequest {
            method: ct::HttpMethod::Post,
            path: oreq::PathParameters::default(),
            query: oreq::QueryParameters::default(),
            headers: oreq::RequestHeaders::default(),
            body: oreq::RequestBody {
                model: "gpt-5".to_string(),
                messages: vec![ct::ChatCompletionMessageParam::Assistant(
                    ct::ChatCompletionAssistantMessageParam {
                        role: ct::ChatCompletionAssistantRole::Assistant,
                        audio: None,
                        content: None,
                        reasoning_content: Some("visible plan".to_string()),
                        reasoning_details: Some(vec![
                            ct::ChatCompletionReasoningDetail {
                                type_: ct::ChatCompletionReasoningDetailType::ReasoningEncrypted,
                                id: Some("enc_1".to_string()),
                                data: Some("ciphertext".to_string()),
                                text: None,
                                signature: Some("sig_enc".to_string()),
                                index: Some(7),
                            },
                            ct::ChatCompletionReasoningDetail {
                                type_: ct::ChatCompletionReasoningDetailType::ReasoningText,
                                id: Some("txt_1".to_string()),
                                data: None,
                                text: Some("detail text".to_string()),
                                signature: Some("sig_text".to_string()),
                                index: Some(8),
                            },
                            ct::ChatCompletionReasoningDetail {
                                type_: ct::ChatCompletionReasoningDetailType::ReasoningSummary,
                                id: Some("sum_1".to_string()),
                                data: None,
                                text: Some("detail summary".to_string()),
                                signature: None,
                                index: Some(9),
                            },
                        ]),
                        function_call: None,
                        name: None,
                        refusal: None,
                        tool_calls: None,
                    },
                )],
                ..oreq::RequestBody::default()
            },
        };

        let converted = OpenAiCreateResponseRequest::try_from(request).unwrap();
        let items = match converted.body.input {
            Some(ot::ResponseInput::Items(items)) => items,
            other => panic!("unexpected input: {other:?}"),
        };
        let reasoning_items = items
            .into_iter()
            .filter_map(|item| match item {
                ot::ResponseInputItem::ReasoningItem(reasoning) => Some(reasoning),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(reasoning_items.len(), 4);
        assert!(reasoning_items.iter().any(|item| {
            item.id.as_deref() == Some("enc_1")
                && item.encrypted_content.as_deref() == Some("ciphertext")
                && item.signature.as_deref() == Some("sig_enc")
        }));
        assert!(reasoning_items.iter().any(|item| {
            item.id.as_deref() == Some("txt_1")
                && item
                    .content
                    .as_ref()
                    .is_some_and(|parts| parts.iter().any(|part| part.text == "detail text"))
                && item.signature.as_deref() == Some("sig_text")
        }));
        assert!(reasoning_items.iter().any(|item| {
            item.id.as_deref() == Some("sum_1")
                && item
                    .summary
                    .iter()
                    .any(|part| part.text == "detail summary")
        }));
    }
}
