use crate::openai::count_tokens::types as ot;
use crate::openai::create_chat_completions::request::{
    OpenAiChatCompletionsRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::create_chat_completions::types as ct;
use crate::openai::create_response::request::OpenAiCreateResponseRequest;
use crate::openai::create_response::types::ResponsePromptCacheRetention;
use crate::transform::openai::count_tokens::utils::{
    openai_function_call_output_content_to_text, openai_input_to_items,
    openai_message_content_to_text, openai_reasoning_summary_to_text,
};
use crate::transform::utils::TransformError;

use super::utils::{
    custom_call_output_to_text, message_content_to_user_content,
    response_reasoning_to_chat_reasoning, response_service_tier_to_chat,
    response_text_to_chat_response_format, response_text_to_chat_verbosity,
    response_tool_choice_to_chat_tool_choice, response_tools_to_chat_tools,
};

fn assistant_message_with_text(text: String) -> ct::ChatCompletionAssistantMessageParam {
    ct::ChatCompletionAssistantMessageParam {
        role: ct::ChatCompletionAssistantRole::Assistant,
        audio: None,
        content: if text.is_empty() {
            None
        } else {
            Some(ct::ChatCompletionAssistantContent::Text(text))
        },
        reasoning_content: None,
        reasoning_details: None,
        function_call: None,
        name: None,
        refusal: None,
        tool_calls: None,
    }
}

fn append_joined_text(target: &mut Option<String>, delta: String) {
    if delta.is_empty() {
        return;
    }

    match target {
        Some(existing) if !existing.is_empty() => {
            existing.push('\n');
            existing.push_str(&delta);
        }
        Some(existing) => existing.push_str(&delta),
        None => *target = Some(delta),
    }
}

fn append_assistant_text(target: &mut ct::ChatCompletionAssistantMessageParam, delta: String) {
    if delta.is_empty() {
        return;
    }

    match target.content.as_mut() {
        Some(ct::ChatCompletionAssistantContent::Text(existing)) if !existing.is_empty() => {
            existing.push('\n');
            existing.push_str(&delta);
        }
        Some(ct::ChatCompletionAssistantContent::Text(existing)) => existing.push_str(&delta),
        Some(ct::ChatCompletionAssistantContent::Parts(parts)) => {
            parts.push(ct::ChatCompletionAssistantContentPart::Text(
                ct::ChatCompletionContentPartText {
                    text: delta,
                    type_: ct::ChatCompletionContentPartTextType::Text,
                },
            ));
        }
        None => {
            target.content = Some(ct::ChatCompletionAssistantContent::Text(delta));
        }
    }
}

fn reasoning_item_to_text(reasoning: &ot::ResponseReasoningItem) -> String {
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

    let text = openai_reasoning_summary_to_text(&reasoning.summary);
    if !text.is_empty() {
        return text;
    }

    String::new()
}

fn reasoning_detail(
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
        details.push(reasoning_detail(
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
            details.push(reasoning_detail(
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
        details.push(reasoning_detail(
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

fn append_reasoning_details(
    target: &mut Option<Vec<ct::ChatCompletionReasoningDetail>>,
    mut details: Vec<ct::ChatCompletionReasoningDetail>,
) {
    if details.is_empty() {
        return;
    }
    target.get_or_insert_with(Vec::new).append(&mut details);
}

fn flush_pending_assistant(
    messages: &mut Vec<ct::ChatCompletionMessageParam>,
    pending_assistant: &mut Option<ct::ChatCompletionAssistantMessageParam>,
) {
    if let Some(message) = pending_assistant.take() {
        messages.push(ct::ChatCompletionMessageParam::Assistant(message));
    }
}

impl TryFrom<OpenAiCreateResponseRequest> for OpenAiChatCompletionsRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiCreateResponseRequest) -> Result<Self, TransformError> {
        let body = value.body;
        let mut messages = Vec::new();
        let mut pending_assistant: Option<ct::ChatCompletionAssistantMessageParam> = None;

        if let Some(instructions) = body.instructions.as_ref().filter(|text| !text.is_empty()) {
            messages.push(ct::ChatCompletionMessageParam::System(
                ct::ChatCompletionSystemMessageParam {
                    content: ct::ChatCompletionTextContent::Text(instructions.clone()),
                    role: ct::ChatCompletionSystemRole::System,
                    name: None,
                },
            ));
        }

        for item in openai_input_to_items(body.input.clone()) {
            match item {
                ot::ResponseInputItem::Message(message) => match message.role {
                    ot::ResponseInputMessageRole::User => {
                        flush_pending_assistant(&mut messages, &mut pending_assistant);
                        messages.push(ct::ChatCompletionMessageParam::User(
                            ct::ChatCompletionUserMessageParam {
                                content: message_content_to_user_content(message.content),
                                role: ct::ChatCompletionUserRole::User,
                                name: None,
                            },
                        ));
                    }
                    ot::ResponseInputMessageRole::Assistant => {
                        let text = openai_message_content_to_text(&message.content);
                        let assistant = pending_assistant
                            .get_or_insert_with(|| assistant_message_with_text(String::new()));
                        append_assistant_text(assistant, text);
                    }
                    ot::ResponseInputMessageRole::System => {
                        flush_pending_assistant(&mut messages, &mut pending_assistant);
                        let text = openai_message_content_to_text(&message.content);
                        messages.push(ct::ChatCompletionMessageParam::System(
                            ct::ChatCompletionSystemMessageParam {
                                content: ct::ChatCompletionTextContent::Text(text),
                                role: ct::ChatCompletionSystemRole::System,
                                name: None,
                            },
                        ));
                    }
                    ot::ResponseInputMessageRole::Developer => {
                        flush_pending_assistant(&mut messages, &mut pending_assistant);
                        let text = openai_message_content_to_text(&message.content);
                        messages.push(ct::ChatCompletionMessageParam::Developer(
                            ct::ChatCompletionDeveloperMessageParam {
                                content: ct::ChatCompletionTextContent::Text(text),
                                role: ct::ChatCompletionDeveloperRole::Developer,
                                name: None,
                            },
                        ));
                    }
                },
                ot::ResponseInputItem::OutputMessage(message) => {
                    let assistant = pending_assistant
                        .get_or_insert_with(|| assistant_message_with_text(String::new()));
                    let mut text_parts = Vec::new();
                    let mut refusal_parts = Vec::new();
                    for part in message.content {
                        match part {
                            ot::ResponseOutputContent::Text(text) => {
                                if !text.text.is_empty() {
                                    text_parts.push(text.text);
                                }
                            }
                            ot::ResponseOutputContent::Refusal(refusal) => {
                                if !refusal.refusal.is_empty() {
                                    refusal_parts.push(refusal.refusal);
                                }
                            }
                        }
                    }

                    append_assistant_text(assistant, text_parts.join("\n"));
                    append_joined_text(&mut assistant.refusal, refusal_parts.join("\n"));
                }
                ot::ResponseInputItem::FunctionToolCall(tool_call) => {
                    let assistant = pending_assistant
                        .get_or_insert_with(|| assistant_message_with_text(String::new()));
                    assistant.tool_calls.get_or_insert_with(Vec::new).push(
                        ct::ChatCompletionMessageToolCall::Function(
                            ct::ChatCompletionMessageFunctionToolCall {
                                id: tool_call.call_id,
                                function: ct::ChatCompletionFunctionCall {
                                    arguments: tool_call.arguments,
                                    name: tool_call.name,
                                },
                                type_: ct::ChatCompletionMessageFunctionToolCallType::Function,
                            },
                        ),
                    );
                }
                ot::ResponseInputItem::CustomToolCall(tool_call) => {
                    let id = tool_call
                        .id
                        .clone()
                        .unwrap_or_else(|| tool_call.call_id.clone());
                    let assistant = pending_assistant
                        .get_or_insert_with(|| assistant_message_with_text(String::new()));
                    assistant.tool_calls.get_or_insert_with(Vec::new).push(
                        ct::ChatCompletionMessageToolCall::Custom(
                            ct::ChatCompletionMessageCustomToolCall {
                                id,
                                custom: ct::ChatCompletionMessageCustomToolCallPayload {
                                    input: tool_call.input,
                                    name: tool_call.name,
                                },
                                type_: ct::ChatCompletionMessageCustomToolCallType::Custom,
                            },
                        ),
                    );
                }
                ot::ResponseInputItem::FunctionCallOutput(output) => {
                    flush_pending_assistant(&mut messages, &mut pending_assistant);
                    let tool_call_id = output.id.unwrap_or(output.call_id);
                    messages.push(ct::ChatCompletionMessageParam::Tool(
                        ct::ChatCompletionToolMessageParam {
                            content: ct::ChatCompletionTextContent::Text(
                                openai_function_call_output_content_to_text(&output.output),
                            ),
                            role: ct::ChatCompletionToolRole::Tool,
                            tool_call_id,
                        },
                    ));
                }
                ot::ResponseInputItem::CustomToolCallOutput(output) => {
                    flush_pending_assistant(&mut messages, &mut pending_assistant);
                    let tool_call_id = output.id.unwrap_or(output.call_id);
                    messages.push(ct::ChatCompletionMessageParam::Tool(
                        ct::ChatCompletionToolMessageParam {
                            content: ct::ChatCompletionTextContent::Text(
                                custom_call_output_to_text(&output.output),
                            ),
                            role: ct::ChatCompletionToolRole::Tool,
                            tool_call_id,
                        },
                    ));
                }
                ot::ResponseInputItem::ReasoningItem(reasoning) => {
                    let assistant = pending_assistant
                        .get_or_insert_with(|| assistant_message_with_text(String::new()));
                    let reasoning_text = reasoning_item_to_text(&reasoning);
                    append_joined_text(&mut assistant.reasoning_content, reasoning_text);
                    append_reasoning_details(
                        &mut assistant.reasoning_details,
                        reasoning_item_to_chat_details(&reasoning),
                    );
                }
                _ => {}
            }
        }

        flush_pending_assistant(&mut messages, &mut pending_assistant);

        let service_tier = body.service_tier.map(response_service_tier_to_chat);
        let response_format = response_text_to_chat_response_format(body.text.as_ref());
        let verbosity = response_text_to_chat_verbosity(body.text.as_ref());
        let prompt_cache_retention = body.prompt_cache_retention.map(|value| match value {
            ResponsePromptCacheRetention::InMemory => {
                ct::ChatCompletionPromptCacheRetention::InMemory
            }
            ResponsePromptCacheRetention::H24 => ct::ChatCompletionPromptCacheRetention::H24,
        });

        Ok(OpenAiChatCompletionsRequest {
            method: ct::HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                messages,
                model: body.model.unwrap_or_default(),
                audio: None,
                frequency_penalty: None,
                function_call: None,
                functions: None,
                logit_bias: None,
                logprobs: None,
                max_completion_tokens: body.max_output_tokens,
                max_tokens: None,
                metadata: body.metadata,
                modalities: None,
                n: None,
                parallel_tool_calls: body.parallel_tool_calls,
                prediction: None,
                presence_penalty: None,
                prompt_cache_key: body.prompt_cache_key,
                prompt_cache_retention,
                reasoning_effort: response_reasoning_to_chat_reasoning(body.reasoning),
                response_format,
                safety_identifier: body.safety_identifier,
                seed: None,
                service_tier,
                stop: None,
                store: body.store,
                stream: None,
                stream_options: None,
                temperature: body.temperature,
                tool_choice: response_tool_choice_to_chat_tool_choice(body.tool_choice),
                tools: response_tools_to_chat_tools(body.tools),
                top_logprobs: body.top_logprobs,
                top_p: body.top_p,
                user: body.user,
                verbosity,
                thinking: None,
                thinking_config: None,
                cached_content: None,
                web_search_options: None,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::create_response::request as rreq;

    #[test]
    fn response_reasoning_items_preserve_chat_reasoning_details() {
        let request = OpenAiCreateResponseRequest {
            body: rreq::RequestBody {
                model: Some("gpt-5".to_string()),
                input: Some(ot::ResponseInput::Items(vec![
                    ot::ResponseInputItem::ReasoningItem(ot::ResponseReasoningItem {
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
                    }),
                ])),
                ..rreq::RequestBody::default()
            },
            ..OpenAiCreateResponseRequest::default()
        };

        let converted = OpenAiChatCompletionsRequest::try_from(request).unwrap();
        let assistant = converted
            .body
            .messages
            .into_iter()
            .find_map(|message| match message {
                ct::ChatCompletionMessageParam::Assistant(message) => Some(message),
                _ => None,
            })
            .expect("assistant message");

        assert_eq!(assistant.reasoning_content.as_deref(), Some("visible text"));
        let details = assistant.reasoning_details.expect("reasoning details");
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
                && detail.signature.as_deref() == Some("sig_rs")
        }));
        assert!(details.iter().any(|detail| {
            matches!(
                detail.type_,
                ct::ChatCompletionReasoningDetailType::ReasoningSummary
            ) && detail.text.as_deref() == Some("summary text")
                && detail.signature.as_deref() == Some("sig_rs")
        }));
    }
}
