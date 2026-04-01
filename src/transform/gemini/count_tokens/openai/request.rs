use crate::gemini::count_tokens::request::GeminiCountTokensRequest;
use crate::gemini::count_tokens::types::{
    GeminiContentRole, GeminiFunctionCallingMode, GeminiLanguage, GeminiOutcome,
};
use crate::openai::count_tokens::request::{
    OpenAiCountTokensRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
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
use crate::transform::gemini::utils::{
    openai_reasoning_effort_from_gemini_thinking, strip_models_prefix,
};
use crate::transform::utils::TransformError;

impl TryFrom<GeminiCountTokensRequest> for OpenAiCountTokensRequest {
    type Error = TransformError;

    fn try_from(value: GeminiCountTokensRequest) -> Result<Self, TransformError> {
        let (raw_model, contents, tools, tool_config, system_instruction, generation_config) =
            if let Some(generate_content_request) = value.body.generate_content_request {
                (
                    generate_content_request.model,
                    generate_content_request.contents,
                    generate_content_request.tools,
                    generate_content_request.tool_config,
                    generate_content_request.system_instruction,
                    generate_content_request.generation_config,
                )
            } else {
                (
                    value.path.model,
                    value.body.contents.unwrap_or_default(),
                    None,
                    None,
                    None,
                    None,
                )
            };

        let model = {
            let stripped = strip_models_prefix(&raw_model);
            if stripped.is_empty() {
                None
            } else {
                Some(stripped)
            }
        };

        let instructions = system_instruction.and_then(|content| {
            let lines = content
                .parts
                .into_iter()
                .filter_map(|part| {
                    if let Some(text) = part.text {
                        let text = text.trim().to_string();
                        if !text.is_empty() {
                            return Some(text);
                        }
                    }
                    if let Some(file_data) = part.file_data
                        && !file_data.file_uri.is_empty()
                    {
                        return Some(file_data.file_uri);
                    }
                    None
                })
                .collect::<Vec<_>>();
            if lines.is_empty() {
                None
            } else {
                Some(lines.join("\n"))
            }
        });

        let mut input_items = Vec::new();
        let mut reasoning_index: u64 = 0;
        let mut tool_call_index: u64 = 0;

        for content in contents {
            let role = match content.role.unwrap_or(GeminiContentRole::User) {
                GeminiContentRole::User => ResponseInputMessageRole::User,
                GeminiContentRole::Model => ResponseInputMessageRole::Assistant,
            };
            let mut message_content = Vec::new();

            for part in content.parts {
                if let Some(text) = part.text
                    && !text.is_empty()
                {
                    if part.thought.unwrap_or(false) {
                        if !message_content.is_empty() {
                            let content = if message_content.len() == 1 {
                                match message_content.into_iter().next() {
                                    Some(ResponseInputContent::Text(text_part)) => {
                                        ResponseInputMessageContent::Text(text_part.text)
                                    }
                                    Some(other) => ResponseInputMessageContent::List(vec![other]),
                                    None => ResponseInputMessageContent::Text(String::new()),
                                }
                            } else {
                                ResponseInputMessageContent::List(message_content)
                            };
                            input_items.push(ResponseInputItem::Message(ResponseInputMessage {
                                content,
                                role: role.clone(),
                                phase: None,
                                status: None,
                                type_: Some(ResponseInputMessageType::Message),
                            }));
                            message_content = Vec::new();
                        }

                        let reasoning_id = part.thought_signature.unwrap_or_else(|| {
                            let id = format!("reasoning_{reasoning_index}");
                            reasoning_index += 1;
                            id
                        });
                        input_items.push(ResponseInputItem::ReasoningItem(ResponseReasoningItem {
                            id: Some(reasoning_id),
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
                        message_content.push(ResponseInputContent::Text(ResponseInputText {
                            text,
                            type_: ResponseInputTextType::InputText,
                        }));
                    }
                }

                if let Some(inline_data) = part.inline_data {
                    if inline_data.mime_type.starts_with("image/") {
                        message_content.push(ResponseInputContent::Image(ResponseInputImage {
                            detail: None,
                            type_: ResponseInputImageType::InputImage,
                            file_id: None,
                            image_url: Some(format!(
                                "data:{};base64,{}",
                                inline_data.mime_type, inline_data.data
                            )),
                        }));
                    } else {
                        message_content.push(ResponseInputContent::File(ResponseInputFile {
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
                        message_content.push(ResponseInputContent::Image(ResponseInputImage {
                            detail: None,
                            type_: ResponseInputImageType::InputImage,
                            file_id: None,
                            image_url: Some(file_data.file_uri),
                        }));
                    } else {
                        message_content.push(ResponseInputContent::File(ResponseInputFile {
                            type_: ResponseInputFileType::InputFile,
                            detail: None,
                            file_data: None,
                            file_id: None,
                            file_url: Some(file_data.file_uri),
                            filename: None,
                        }));
                    }
                }

                if let Some(code) = part.executable_code {
                    let language = match code.language {
                        GeminiLanguage::Python => "python",
                        GeminiLanguage::LanguageUnspecified => "text",
                    };
                    message_content.push(ResponseInputContent::Text(ResponseInputText {
                        text: format!("```{language}\n{}\n```", code.code),
                        type_: ResponseInputTextType::InputText,
                    }));
                }

                if let Some(result) = part.code_execution_result {
                    let outcome = match result.outcome {
                        GeminiOutcome::OutcomeUnspecified => "outcome_unspecified",
                        GeminiOutcome::OutcomeOk => "outcome_ok",
                        GeminiOutcome::OutcomeFailed => "outcome_failed",
                        GeminiOutcome::OutcomeDeadlineExceeded => "outcome_deadline_exceeded",
                    };
                    let text = match result.output {
                        Some(output) if !output.is_empty() => {
                            format!("code_execution_result:{outcome}\n{output}")
                        }
                        _ => format!("code_execution_result:{outcome}"),
                    };
                    message_content.push(ResponseInputContent::Text(ResponseInputText {
                        text,
                        type_: ResponseInputTextType::InputText,
                    }));
                }

                if let Some(function_call) = part.function_call {
                    if !message_content.is_empty() {
                        let content = if message_content.len() == 1 {
                            match message_content.into_iter().next() {
                                Some(ResponseInputContent::Text(text_part)) => {
                                    ResponseInputMessageContent::Text(text_part.text)
                                }
                                Some(other) => ResponseInputMessageContent::List(vec![other]),
                                None => ResponseInputMessageContent::Text(String::new()),
                            }
                        } else {
                            ResponseInputMessageContent::List(message_content)
                        };
                        input_items.push(ResponseInputItem::Message(ResponseInputMessage {
                            content,
                            role: role.clone(),
                            phase: None,
                            status: None,
                            type_: Some(ResponseInputMessageType::Message),
                        }));
                        message_content = Vec::new();
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
                    if !message_content.is_empty() {
                        let content = if message_content.len() == 1 {
                            match message_content.into_iter().next() {
                                Some(ResponseInputContent::Text(text_part)) => {
                                    ResponseInputMessageContent::Text(text_part.text)
                                }
                                Some(other) => ResponseInputMessageContent::List(vec![other]),
                                None => ResponseInputMessageContent::Text(String::new()),
                            }
                        } else {
                            ResponseInputMessageContent::List(message_content)
                        };
                        input_items.push(ResponseInputItem::Message(ResponseInputMessage {
                            content,
                            role: role.clone(),
                            phase: None,
                            status: None,
                            type_: Some(ResponseInputMessageType::Message),
                        }));
                        message_content = Vec::new();
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

            if !message_content.is_empty() {
                let content = if message_content.len() == 1 {
                    match message_content.into_iter().next() {
                        Some(ResponseInputContent::Text(text_part)) => {
                            ResponseInputMessageContent::Text(text_part.text)
                        }
                        Some(other) => ResponseInputMessageContent::List(vec![other]),
                        None => ResponseInputMessageContent::Text(String::new()),
                    }
                } else {
                    ResponseInputMessageContent::List(message_content)
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

        let tools = tools.and_then(|tool_defs| {
            let mut mapped = Vec::new();
            for tool in tool_defs {
                if let Some(function_declarations) = tool.function_declarations {
                    for declaration in function_declarations {
                        let parameters = declaration
                            .parameters_json_schema
                            .and_then(|value| match value {
                                serde_json::Value::Object(map) => Some(map.into_iter().collect()),
                                _ => None,
                            })
                            .unwrap_or_default();
                        mapped.push(ResponseTool::Function(ResponseFunctionTool {
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
                    mapped.push(ResponseTool::FileSearch(ResponseFileSearchTool {
                        type_: ResponseFileSearchToolType::FileSearch,
                        vector_store_ids: file_search.file_search_store_names,
                        filters: None,
                        max_num_results: file_search.top_k.and_then(|v| u32::try_from(v).ok()),
                        ranking_options: None,
                    }));
                }

                if tool.computer_use.is_some() {
                    mapped.push(ResponseTool::Computer(ResponseComputerTool {
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
                    mapped.push(ResponseTool::WebSearch(ResponseWebSearchTool {
                        type_: ResponseWebSearchToolType::WebSearch,
                        filters: None,
                        search_context_size: None,
                        user_location: None,
                    }));
                }

                if tool.code_execution.is_some() {
                    mapped.push(ResponseTool::CodeInterpreter(ResponseCodeInterpreterTool {
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

            if mapped.is_empty() {
                None
            } else {
                Some(mapped)
            }
        });

        let tool_choice = tool_config
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

        let (reasoning, text) = if let Some(config) = generation_config {
            let reasoning = config
                .thinking_config
                .as_ref()
                .and_then(openai_reasoning_effort_from_gemini_thinking)
                .map(|effort| ResponseReasoning {
                    effort: Some(effort),
                    generate_summary: None,
                    summary: None,
                });

            let schema_value = config
                .response_json_schema
                .or(config.response_json_schema_legacy)
                .or_else(|| {
                    config
                        .response_schema
                        .and_then(|schema| serde_json::to_value(schema).ok())
                });
            let schema = schema_value.and_then(|value| match value {
                serde_json::Value::Object(map) => Some(map.into_iter().collect()),
                _ => None,
            });
            let mime = config.response_mime_type.as_deref().unwrap_or_default();
            let format = match mime {
                "application/json" => Some(if let Some(schema) = schema {
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
                "text/plain" => Some(ResponseTextFormatConfig::Text(ResponseFormatText {
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
            let text = format.map(|format| ResponseTextConfig {
                format: Some(format),
                verbosity: None,
            });
            (reasoning, text)
        } else {
            (None, None)
        };

        Ok(Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                conversation: None,
                input,
                instructions,
                model,
                parallel_tool_calls: None,
                previous_response_id: None,
                reasoning,
                text,
                tool_choice,
                tools,
                truncation: None,
            },
        })
    }
}
