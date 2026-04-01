use crate::gemini::count_tokens::types::GeminiContentRole;
use crate::gemini::generate_content::request::GeminiGenerateContentRequest;
use crate::gemini::generate_content::types::GeminiFunctionCallingMode;
use crate::openai::create_chat_completions::request::{
    OpenAiChatCompletionsRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::create_chat_completions::types::{
    ChatCompletionAssistantContent, ChatCompletionAssistantMessageParam,
    ChatCompletionAssistantRole, ChatCompletionContentPart, ChatCompletionContentPartFile,
    ChatCompletionContentPartFileType, ChatCompletionContentPartImage,
    ChatCompletionContentPartImageType, ChatCompletionContentPartText,
    ChatCompletionContentPartTextType, ChatCompletionFileInput, ChatCompletionFunctionCall,
    ChatCompletionFunctionDefinition, ChatCompletionFunctionTool, ChatCompletionFunctionToolType,
    ChatCompletionImageUrl, ChatCompletionMessageFunctionToolCall,
    ChatCompletionMessageFunctionToolCallType, ChatCompletionMessageParam,
    ChatCompletionMessageToolCall, ChatCompletionNamedFunction, ChatCompletionNamedToolChoice,
    ChatCompletionNamedToolChoiceType, ChatCompletionResponseFormat,
    ChatCompletionResponseFormatJsonObject, ChatCompletionResponseFormatJsonObjectType,
    ChatCompletionResponseFormatJsonSchema, ChatCompletionResponseFormatJsonSchemaConfig,
    ChatCompletionResponseFormatJsonSchemaType, ChatCompletionResponseFormatText,
    ChatCompletionResponseFormatTextType, ChatCompletionStop, ChatCompletionSystemMessageParam,
    ChatCompletionSystemRole, ChatCompletionTextContent, ChatCompletionTool,
    ChatCompletionToolChoiceMode, ChatCompletionToolChoiceOption, ChatCompletionToolMessageParam,
    ChatCompletionToolRole, ChatCompletionUserContent, ChatCompletionUserMessageParam,
    ChatCompletionUserRole, HttpMethod,
};
use crate::transform::gemini::utils::{
    openai_chat_reasoning_effort_from_gemini_thinking, strip_models_prefix,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiGenerateContentRequest> for OpenAiChatCompletionsRequest {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentRequest) -> Result<Self, TransformError> {
        let body = value.body;
        let model = strip_models_prefix(&value.path.model);

        let mut messages = Vec::new();
        if let Some(system_instruction) = body.system_instruction {
            let system_text = system_instruction
                .parts
                .into_iter()
                .filter_map(|part| part.text)
                .filter(|text| !text.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
            if !system_text.is_empty() {
                messages.push(ChatCompletionMessageParam::System(
                    ChatCompletionSystemMessageParam {
                        content: ChatCompletionTextContent::Text(system_text),
                        role: ChatCompletionSystemRole::System,
                        name: None,
                    },
                ));
            }
        }

        let mut tool_output_messages = Vec::new();
        let mut tool_call_index = 0u64;
        for content in body.contents {
            let role = content.role.unwrap_or(GeminiContentRole::User);
            let mut text_parts = Vec::new();
            let mut user_parts = Vec::new();
            let mut tool_calls = Vec::new();

            for part in content.parts {
                if let Some(text) = part.text
                    && !text.is_empty()
                {
                    text_parts.push(text.clone());
                    if matches!(role, GeminiContentRole::User) {
                        user_parts.push(ChatCompletionContentPart::Text(
                            ChatCompletionContentPartText {
                                text,
                                type_: ChatCompletionContentPartTextType::Text,
                            },
                        ));
                    }
                }

                if let Some(inline_data) = part.inline_data {
                    if inline_data.mime_type.starts_with("image/") {
                        user_parts.push(ChatCompletionContentPart::Image(
                            ChatCompletionContentPartImage {
                                image_url: ChatCompletionImageUrl {
                                    url: format!(
                                        "data:{};base64,{}",
                                        inline_data.mime_type, inline_data.data
                                    ),
                                    detail: None,
                                },
                                type_: ChatCompletionContentPartImageType::ImageUrl,
                            },
                        ));
                    } else {
                        user_parts.push(ChatCompletionContentPart::File(
                            ChatCompletionContentPartFile {
                                file: ChatCompletionFileInput {
                                    file_data: Some(inline_data.data),
                                    file_id: None,
                                    file_url: None,
                                    filename: Some(inline_data.mime_type),
                                },
                                type_: ChatCompletionContentPartFileType::File,
                            },
                        ));
                    }
                }

                if let Some(file_data) = part.file_data {
                    if file_data
                        .mime_type
                        .as_deref()
                        .unwrap_or_default()
                        .starts_with("image/")
                    {
                        user_parts.push(ChatCompletionContentPart::Image(
                            ChatCompletionContentPartImage {
                                image_url: ChatCompletionImageUrl {
                                    url: file_data.file_uri,
                                    detail: None,
                                },
                                type_: ChatCompletionContentPartImageType::ImageUrl,
                            },
                        ));
                    } else {
                        user_parts.push(ChatCompletionContentPart::File(
                            ChatCompletionContentPartFile {
                                file: ChatCompletionFileInput {
                                    file_data: None,
                                    file_id: None,
                                    file_url: Some(file_data.file_uri),
                                    filename: None,
                                },
                                type_: ChatCompletionContentPartFileType::File,
                            },
                        ));
                    }
                }

                if let Some(function_call) = part.function_call {
                    let id = function_call.id.unwrap_or_else(|| {
                        let id = format!("tool_call_{tool_call_index}");
                        tool_call_index += 1;
                        id
                    });
                    let arguments = function_call
                        .args
                        .and_then(|args| serde_json::to_string(&args).ok())
                        .unwrap_or_else(|| "{}".to_string());
                    tool_calls.push(ChatCompletionMessageToolCall::Function(
                        ChatCompletionMessageFunctionToolCall {
                            id,
                            function: ChatCompletionFunctionCall {
                                arguments,
                                name: function_call.name,
                            },
                            type_: ChatCompletionMessageFunctionToolCallType::Function,
                        },
                    ));
                }

                if let Some(function_response) = part.function_response {
                    let output = serde_json::to_string(&function_response.response)
                        .unwrap_or_else(|_| "{}".to_string());
                    let tool_call_id = function_response.id.unwrap_or(function_response.name);
                    tool_output_messages.push(ChatCompletionMessageParam::Tool(
                        ChatCompletionToolMessageParam {
                            content: ChatCompletionTextContent::Text(output),
                            role: ChatCompletionToolRole::Tool,
                            tool_call_id,
                        },
                    ));
                }
            }

            match role {
                GeminiContentRole::User => {
                    if user_parts.is_empty() {
                        messages.push(ChatCompletionMessageParam::User(
                            ChatCompletionUserMessageParam {
                                content: ChatCompletionUserContent::Text(text_parts.join("\n")),
                                role: ChatCompletionUserRole::User,
                                name: None,
                            },
                        ));
                    } else {
                        messages.push(ChatCompletionMessageParam::User(
                            ChatCompletionUserMessageParam {
                                content: ChatCompletionUserContent::Parts(user_parts),
                                role: ChatCompletionUserRole::User,
                                name: None,
                            },
                        ));
                    }
                }
                GeminiContentRole::Model => {
                    messages.push(ChatCompletionMessageParam::Assistant(
                        ChatCompletionAssistantMessageParam {
                            role: ChatCompletionAssistantRole::Assistant,
                            audio: None,
                            content: if text_parts.is_empty() {
                                None
                            } else {
                                Some(ChatCompletionAssistantContent::Text(text_parts.join("\n")))
                            },
                            reasoning_content: None,
                            function_call: None,
                            name: None,
                            refusal: None,
                            tool_calls: if tool_calls.is_empty() {
                                None
                            } else {
                                Some(tool_calls)
                            },
                        },
                    ));
                }
            }
        }
        messages.extend(tool_output_messages);

        let tools = body.tools.and_then(|tool_defs| {
            let mut mapped = Vec::new();
            for tool in tool_defs {
                if let Some(function_declarations) = tool.function_declarations {
                    for declaration in function_declarations {
                        let parameters = declaration
                            .parameters_json_schema
                            .and_then(|value| {
                                serde_json::from_value::<crate::openai::create_chat_completions::types::FunctionParameters>(value).ok()
                            });
                        mapped.push(ChatCompletionTool::Function(ChatCompletionFunctionTool {
                            function: ChatCompletionFunctionDefinition {
                                name: declaration.name,
                                description: if declaration.description.is_empty() {
                                    None
                                } else {
                                    Some(declaration.description)
                                },
                                parameters,
                                strict: None,
                            },
                            type_: ChatCompletionFunctionToolType::Function,
                        }));
                    }
                }

                if tool.code_execution.is_some() {
                    mapped.push(ChatCompletionTool::Custom(
                        crate::openai::create_chat_completions::types::ChatCompletionCustomTool {
                            custom: crate::openai::create_chat_completions::types::ChatCompletionCustomToolSpec {
                                name: "code_execution".to_string(),
                                description: None,
                                format: None,
                            },
                            type_: crate::openai::create_chat_completions::types::ChatCompletionCustomToolType::Custom,
                        },
                    ));
                }
            }

            if mapped.is_empty() {
                None
            } else {
                Some(mapped)
            }
        });

        let tool_choice = body
            .tool_config
            .and_then(|config| config.function_calling_config)
            .map(|config| {
                if let Some(name) = config
                    .allowed_function_names
                    .as_ref()
                    .and_then(|names| names.first())
                    .cloned()
                {
                    return ChatCompletionToolChoiceOption::NamedFunction(
                        ChatCompletionNamedToolChoice {
                            function: ChatCompletionNamedFunction { name },
                            type_: ChatCompletionNamedToolChoiceType::Function,
                        },
                    );
                }
                match config
                    .mode
                    .unwrap_or(GeminiFunctionCallingMode::ModeUnspecified)
                {
                    GeminiFunctionCallingMode::Auto
                    | GeminiFunctionCallingMode::ModeUnspecified => {
                        ChatCompletionToolChoiceOption::Mode(ChatCompletionToolChoiceMode::Auto)
                    }
                    GeminiFunctionCallingMode::Any | GeminiFunctionCallingMode::Validated => {
                        ChatCompletionToolChoiceOption::Mode(ChatCompletionToolChoiceMode::Required)
                    }
                    GeminiFunctionCallingMode::None => {
                        ChatCompletionToolChoiceOption::Mode(ChatCompletionToolChoiceMode::None)
                    }
                }
            });

        let (max_completion_tokens, reasoning_effort, response_format, stop, temperature, top_p) =
            if let Some(config) = body.generation_config {
                let max_completion_tokens = config.max_output_tokens.map(u64::from);
                let reasoning_effort = config
                    .thinking_config
                    .as_ref()
                    .and_then(openai_chat_reasoning_effort_from_gemini_thinking);

                let schema = config
                    .response_json_schema
                    .or(config.response_json_schema_legacy)
                    .or_else(|| {
                        config
                            .response_schema
                            .and_then(|schema| serde_json::to_value(schema).ok())
                    })
                    .and_then(|value| {
                        serde_json::from_value::<
                            crate::openai::create_chat_completions::types::JsonObject,
                        >(value)
                        .ok()
                    });
                let response_format = match config.response_mime_type.as_deref() {
                    Some("application/json") => Some(if let Some(schema) = schema {
                        ChatCompletionResponseFormat::JsonSchema(
                            ChatCompletionResponseFormatJsonSchema {
                                json_schema: ChatCompletionResponseFormatJsonSchemaConfig {
                                    name: "output".to_string(),
                                    description: None,
                                    schema: Some(schema),
                                    strict: None,
                                },
                                type_: ChatCompletionResponseFormatJsonSchemaType::JsonSchema,
                            },
                        )
                    } else {
                        ChatCompletionResponseFormat::JsonObject(
                            ChatCompletionResponseFormatJsonObject {
                                type_: ChatCompletionResponseFormatJsonObjectType::JsonObject,
                            },
                        )
                    }),
                    Some("text/plain") => Some(ChatCompletionResponseFormat::Text(
                        ChatCompletionResponseFormatText {
                            type_: ChatCompletionResponseFormatTextType::Text,
                        },
                    )),
                    _ => schema.map(|schema| {
                        ChatCompletionResponseFormat::JsonSchema(
                            ChatCompletionResponseFormatJsonSchema {
                                json_schema: ChatCompletionResponseFormatJsonSchemaConfig {
                                    name: "output".to_string(),
                                    description: None,
                                    schema: Some(schema),
                                    strict: None,
                                },
                                type_: ChatCompletionResponseFormatJsonSchemaType::JsonSchema,
                            },
                        )
                    }),
                };

                let stop = config.stop_sequences.map(|items| {
                    if items.len() == 1 {
                        ChatCompletionStop::Single(items[0].clone())
                    } else {
                        ChatCompletionStop::Multiple(items)
                    }
                });

                (
                    max_completion_tokens,
                    reasoning_effort,
                    response_format,
                    stop,
                    config.temperature,
                    config.top_p,
                )
            } else {
                (None, None, None, None, None, None)
            };

        Ok(OpenAiChatCompletionsRequest {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                messages,
                model,
                max_completion_tokens,
                reasoning_effort,
                response_format,
                stop,
                stream: None,
                temperature,
                tool_choice,
                tools,
                top_p,
                ..RequestBody::default()
            },
        })
    }
}
