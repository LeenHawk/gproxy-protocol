use crate::gemini::count_tokens::types::GeminiContentRole;
use crate::gemini::generate_content::request::GeminiGenerateContentRequest;
use crate::gemini::generate_content::types::GeminiFunctionCallingMode;
use crate::openai::count_tokens::types::{
    HttpMethod, ResponseCodeInterpreterContainer, ResponseCodeInterpreterTool,
    ResponseCodeInterpreterToolAuto, ResponseCodeInterpreterToolAutoType,
    ResponseCodeInterpreterToolType, ResponseComputerEnvironment, ResponseComputerTool,
    ResponseComputerToolType, ResponseFileSearchTool, ResponseFileSearchToolType,
    ResponseFormatJsonObject, ResponseFormatJsonObjectType, ResponseFormatText,
    ResponseFormatTextJsonSchemaConfig, ResponseFormatTextJsonSchemaConfigType,
    ResponseFormatTextType, ResponseFunctionCallOutput, ResponseFunctionCallOutputContent,
    ResponseFunctionCallOutputType, ResponseFunctionTool, ResponseFunctionToolCall,
    ResponseFunctionToolCallType, ResponseInput, ResponseInputContent, ResponseInputFile,
    ResponseInputFileType, ResponseInputImage, ResponseInputImageType, ResponseInputItem,
    ResponseInputMessage, ResponseInputMessageContent, ResponseInputMessageRole,
    ResponseInputMessageType, ResponseInputText, ResponseInputTextType, ResponseReasoning,
    ResponseReasoningItem, ResponseReasoningItemType, ResponseSummaryTextContent,
    ResponseSummaryTextContentType, ResponseTextConfig, ResponseTextFormatConfig, ResponseTool,
    ResponseToolChoice, ResponseToolChoiceFunction, ResponseToolChoiceFunctionType,
    ResponseToolChoiceOptions, ResponseWebSearchTool, ResponseWebSearchToolType,
};
use crate::openai::create_response::request::{
    OpenAiCreateResponseRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::transform::gemini::utils::{
    gemini_content_to_text, openai_reasoning_effort_from_gemini_thinking, strip_models_prefix,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiGenerateContentRequest> for OpenAiCreateResponseRequest {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentRequest) -> Result<Self, TransformError> {
        let body = value.body;

        let instructions = body
            .system_instruction
            .as_ref()
            .map(gemini_content_to_text)
            .filter(|text| !text.is_empty());

        let mut input_items = Vec::new();
        let mut reasoning_index = 0u64;
        let mut tool_call_index = 0u64;
        for content in body.contents {
            let role = match content.role.unwrap_or(GeminiContentRole::User) {
                GeminiContentRole::User => ResponseInputMessageRole::User,
                GeminiContentRole::Model => ResponseInputMessageRole::Assistant,
            };
            let mut message_parts = Vec::new();

            for part in content.parts {
                if let Some(text) = part.text
                    && !text.is_empty()
                {
                    if part.thought.unwrap_or(false) {
                        if !message_parts.is_empty() {
                            let content = if message_parts.len() == 1 {
                                match message_parts.into_iter().next() {
                                    Some(ResponseInputContent::Text(text_part)) => {
                                        ResponseInputMessageContent::Text(text_part.text)
                                    }
                                    Some(other) => ResponseInputMessageContent::List(vec![other]),
                                    None => ResponseInputMessageContent::Text(String::new()),
                                }
                            } else {
                                ResponseInputMessageContent::List(message_parts)
                            };
                            input_items.push(ResponseInputItem::Message(ResponseInputMessage {
                                content,
                                role: role.clone(),
                                phase: None,
                                status: None,
                                type_: Some(ResponseInputMessageType::Message),
                            }));
                            message_parts = Vec::new();
                        }

                        let id = part.thought_signature.unwrap_or_else(|| {
                            let id = format!("reasoning_{reasoning_index}");
                            reasoning_index += 1;
                            id
                        });
                        input_items.push(ResponseInputItem::ReasoningItem(ResponseReasoningItem {
                            id: Some(id),
                            summary: vec![ResponseSummaryTextContent {
                                text,
                                type_: ResponseSummaryTextContentType::SummaryText,
                            }],
                            type_: ResponseReasoningItemType::Reasoning,
                            content: None,
                            encrypted_content: None,
                            status: None,
                        }));
                    } else {
                        message_parts.push(ResponseInputContent::Text(ResponseInputText {
                            text,
                            type_: ResponseInputTextType::InputText,
                        }));
                    }
                }

                if let Some(inline_data) = part.inline_data {
                    if inline_data.mime_type.starts_with("image/") {
                        message_parts.push(ResponseInputContent::Image(ResponseInputImage {
                            detail: None,
                            type_: ResponseInputImageType::InputImage,
                            file_id: None,
                            image_url: Some(format!(
                                "data:{};base64,{}",
                                inline_data.mime_type, inline_data.data
                            )),
                        }));
                    } else {
                        message_parts.push(ResponseInputContent::File(ResponseInputFile {
                            type_: ResponseInputFileType::InputFile,
                            detail: None,
                            file_data: Some(inline_data.data),
                            file_id: None,
                            file_url: None,
                            filename: Some(inline_data.mime_type),
                        }));
                    }
                }

                if let Some(file_data) = part.file_data {
                    if file_data.file_uri.is_empty() {
                        continue;
                    }
                    if file_data
                        .mime_type
                        .as_deref()
                        .unwrap_or_default()
                        .starts_with("image/")
                    {
                        message_parts.push(ResponseInputContent::Image(ResponseInputImage {
                            detail: None,
                            type_: ResponseInputImageType::InputImage,
                            file_id: None,
                            image_url: Some(file_data.file_uri),
                        }));
                    } else {
                        message_parts.push(ResponseInputContent::File(ResponseInputFile {
                            type_: ResponseInputFileType::InputFile,
                            detail: None,
                            file_data: None,
                            file_id: None,
                            file_url: Some(file_data.file_uri),
                            filename: None,
                        }));
                    }
                }

                if let Some(function_call) = part.function_call {
                    if !message_parts.is_empty() {
                        let content = if message_parts.len() == 1 {
                            match message_parts.into_iter().next() {
                                Some(ResponseInputContent::Text(text_part)) => {
                                    ResponseInputMessageContent::Text(text_part.text)
                                }
                                Some(other) => ResponseInputMessageContent::List(vec![other]),
                                None => ResponseInputMessageContent::Text(String::new()),
                            }
                        } else {
                            ResponseInputMessageContent::List(message_parts)
                        };
                        input_items.push(ResponseInputItem::Message(ResponseInputMessage {
                            content,
                            role: role.clone(),
                            phase: None,
                            status: None,
                            type_: Some(ResponseInputMessageType::Message),
                        }));
                        message_parts = Vec::new();
                    }

                    let call_id = function_call.id.unwrap_or_else(|| {
                        let id = format!("call_{tool_call_index}");
                        tool_call_index += 1;
                        id
                    });
                    let arguments = function_call
                        .args
                        .and_then(|args| serde_json::to_string(&args).ok())
                        .unwrap_or_else(|| "{}".to_string());
                    input_items.push(ResponseInputItem::FunctionToolCall(
                        ResponseFunctionToolCall {
                            arguments,
                            call_id: call_id.clone(),
                            name: function_call.name,
                            type_: ResponseFunctionToolCallType::FunctionCall,
                            id: Some(call_id),
                            status: None,
                        },
                    ));
                }

                if let Some(function_response) = part.function_response {
                    if !message_parts.is_empty() {
                        let content = if message_parts.len() == 1 {
                            match message_parts.into_iter().next() {
                                Some(ResponseInputContent::Text(text_part)) => {
                                    ResponseInputMessageContent::Text(text_part.text)
                                }
                                Some(other) => ResponseInputMessageContent::List(vec![other]),
                                None => ResponseInputMessageContent::Text(String::new()),
                            }
                        } else {
                            ResponseInputMessageContent::List(message_parts)
                        };
                        input_items.push(ResponseInputItem::Message(ResponseInputMessage {
                            content,
                            role: role.clone(),
                            phase: None,
                            status: None,
                            type_: Some(ResponseInputMessageType::Message),
                        }));
                        message_parts = Vec::new();
                    }

                    let call_id = function_response
                        .id
                        .unwrap_or_else(|| function_response.name.clone());
                    let output = match serde_json::to_string(&function_response.response) {
                        Ok(text) if !text.is_empty() => {
                            ResponseFunctionCallOutputContent::Text(text)
                        }
                        _ => ResponseFunctionCallOutputContent::Text("{}".to_string()),
                    };
                    input_items.push(ResponseInputItem::FunctionCallOutput(
                        ResponseFunctionCallOutput {
                            call_id,
                            output,
                            type_: ResponseFunctionCallOutputType::FunctionCallOutput,
                            id: None,
                            status: None,
                        },
                    ));
                }
            }

            if !message_parts.is_empty() {
                let content = if message_parts.len() == 1 {
                    match message_parts.into_iter().next() {
                        Some(ResponseInputContent::Text(text_part)) => {
                            ResponseInputMessageContent::Text(text_part.text)
                        }
                        Some(other) => ResponseInputMessageContent::List(vec![other]),
                        None => ResponseInputMessageContent::Text(String::new()),
                    }
                } else {
                    ResponseInputMessageContent::List(message_parts)
                };
                input_items.push(ResponseInputItem::Message(ResponseInputMessage {
                    content,
                    role,
                    phase: None,
                    status: None,
                    type_: Some(ResponseInputMessageType::Message),
                }));
            }
        }
        let input = if input_items.is_empty() {
            None
        } else {
            Some(ResponseInput::Items(input_items))
        };

        let tools = body.tools.and_then(|tools| {
            let mut converted_tools = Vec::new();
            for tool in tools {
                if let Some(function_declarations) = tool.function_declarations {
                    for declaration in function_declarations {
                        let parameters = declaration
                            .parameters_json_schema
                            .and_then(|value| {
                                serde_json::from_value::<crate::openai::count_tokens::types::JsonObject>(value).ok()
                            })
                            .unwrap_or_default();
                        converted_tools.push(ResponseTool::Function(ResponseFunctionTool {
                            name: declaration.name,
                            parameters,
                            strict: None,
                            type_: crate::openai::count_tokens::types::ResponseFunctionToolType::Function,
                            defer_loading: None,
                            description: if declaration.description.is_empty() {
                                None
                            } else {
                                Some(declaration.description)
                            },
                        }));
                    }
                }

                if let Some(file_search) = tool.file_search {
                    converted_tools.push(ResponseTool::FileSearch(ResponseFileSearchTool {
                        type_: ResponseFileSearchToolType::FileSearch,
                        vector_store_ids: file_search.file_search_store_names,
                        filters: None,
                        max_num_results: file_search.top_k.and_then(|v| u32::try_from(v).ok()),
                        ranking_options: None,
                    }));
                }

                if tool.computer_use.is_some() {
                    converted_tools.push(ResponseTool::Computer(ResponseComputerTool {
                        display_height: Some(1024),
                        display_width: Some(1024),
                        environment: Some(ResponseComputerEnvironment::Browser),
                        type_: ResponseComputerToolType::ComputerUsePreview,
                    }));
                }

                if tool.google_search.is_some()
                    || tool.google_search_retrieval.is_some()
                    || tool.url_context.is_some()
                    || tool.google_maps.is_some()
                {
                    converted_tools.push(ResponseTool::WebSearch(ResponseWebSearchTool {
                        type_: ResponseWebSearchToolType::WebSearch,
                        filters: None,
                        search_context_size: None,
                        user_location: None,
                    }));
                }

                if tool.code_execution.is_some() {
                    converted_tools.push(ResponseTool::CodeInterpreter(ResponseCodeInterpreterTool {
                        container: ResponseCodeInterpreterContainer::Auto(
                            ResponseCodeInterpreterToolAuto {
                                type_: ResponseCodeInterpreterToolAutoType::Auto,
                                file_ids: None,
                                memory_limit: None,
                                network_policy: None,
                            },
                        ),
                        type_: ResponseCodeInterpreterToolType::CodeInterpreter,
                    }));
                }
            }
            if converted_tools.is_empty() {
                None
            } else {
                Some(converted_tools)
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
                    return ResponseToolChoice::Function(ResponseToolChoiceFunction {
                        name,
                        type_: ResponseToolChoiceFunctionType::Function,
                    });
                }
                match config
                    .mode
                    .unwrap_or(GeminiFunctionCallingMode::ModeUnspecified)
                {
                    GeminiFunctionCallingMode::Auto
                    | GeminiFunctionCallingMode::ModeUnspecified => {
                        ResponseToolChoice::Options(ResponseToolChoiceOptions::Auto)
                    }
                    GeminiFunctionCallingMode::Any | GeminiFunctionCallingMode::Validated => {
                        ResponseToolChoice::Options(ResponseToolChoiceOptions::Required)
                    }
                    GeminiFunctionCallingMode::None => {
                        ResponseToolChoice::Options(ResponseToolChoiceOptions::None)
                    }
                }
            });

        let max_output_tokens = body
            .generation_config
            .as_ref()
            .and_then(|config| config.max_output_tokens)
            .map(u64::from);
        let temperature = body
            .generation_config
            .as_ref()
            .and_then(|config| config.temperature);
        let top_p = body
            .generation_config
            .as_ref()
            .and_then(|config| config.top_p);

        let reasoning = body
            .generation_config
            .as_ref()
            .and_then(|config| config.thinking_config.as_ref())
            .and_then(openai_reasoning_effort_from_gemini_thinking)
            .map(|effort| ResponseReasoning {
                effort: Some(effort),
                generate_summary: None,
                summary: None,
            });

        let text = body.generation_config.as_ref().and_then(|config| {
            let schema = config
                .response_json_schema
                .clone()
                .or(config.response_json_schema_legacy.clone())
                .or_else(|| {
                    config
                        .response_schema
                        .as_ref()
                        .and_then(|schema| serde_json::to_value(schema).ok())
                })
                .and_then(|value| {
                    serde_json::from_value::<crate::openai::count_tokens::types::JsonObject>(value)
                        .ok()
                });

            let format = match config.response_mime_type.as_deref() {
                Some("application/json") => Some(if let Some(schema) = schema {
                    ResponseTextFormatConfig::JsonSchema(ResponseFormatTextJsonSchemaConfig {
                        name: "output".to_string(),
                        schema,
                        type_: ResponseFormatTextJsonSchemaConfigType::JsonSchema,
                        description: None,
                        strict: None,
                    })
                } else {
                    ResponseTextFormatConfig::JsonObject(ResponseFormatJsonObject {
                        type_: ResponseFormatJsonObjectType::JsonObject,
                    })
                }),
                Some("text/plain") => Some(ResponseTextFormatConfig::Text(ResponseFormatText {
                    type_: ResponseFormatTextType::Text,
                })),
                _ => schema.map(|schema| {
                    ResponseTextFormatConfig::JsonSchema(ResponseFormatTextJsonSchemaConfig {
                        name: "output".to_string(),
                        schema,
                        type_: ResponseFormatTextJsonSchemaConfigType::JsonSchema,
                        description: None,
                        strict: None,
                    })
                }),
            };

            format.map(|format| ResponseTextConfig {
                format: Some(format),
                verbosity: None,
            })
        });

        Ok(Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                input,
                instructions,
                max_output_tokens,
                model: Some(strip_models_prefix(&value.path.model)),
                reasoning,
                stream: None,
                temperature,
                text,
                tool_choice,
                tools,
                top_p,
                ..RequestBody::default()
            },
        })
    }
}
