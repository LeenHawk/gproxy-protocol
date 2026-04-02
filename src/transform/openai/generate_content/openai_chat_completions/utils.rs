use crate::openai::count_tokens::types as ot;
use crate::openai::create_chat_completions::types as ct;

fn chat_image_detail_to_response(
    detail: Option<ct::ChatCompletionImageDetail>,
) -> Option<ot::ResponseInputImageDetail> {
    detail.map(|value| match value {
        ct::ChatCompletionImageDetail::Auto => ot::ResponseInputImageDetail::Auto,
        ct::ChatCompletionImageDetail::Low => ot::ResponseInputImageDetail::Low,
        ct::ChatCompletionImageDetail::High => ot::ResponseInputImageDetail::High,
        ct::ChatCompletionImageDetail::Original => ot::ResponseInputImageDetail::Original,
    })
}

fn chat_content_part_to_response_input_content(
    part: ct::ChatCompletionContentPart,
) -> ot::ResponseInputContent {
    match part {
        ct::ChatCompletionContentPart::Text(part) => {
            ot::ResponseInputContent::Text(ot::ResponseInputText {
                text: part.text,
                type_: ot::ResponseInputTextType::InputText,
            })
        }
        ct::ChatCompletionContentPart::Image(part) => {
            let (file_id, image_url) = if part.image_url.url.starts_with("file:") {
                (
                    Some(part.image_url.url.trim_start_matches("file:").to_string()),
                    None,
                )
            } else {
                (None, Some(part.image_url.url))
            };
            ot::ResponseInputContent::Image(ot::ResponseInputImage {
                detail: chat_image_detail_to_response(part.image_url.detail),
                type_: ot::ResponseInputImageType::InputImage,
                file_id,
                image_url,
            })
        }
        ct::ChatCompletionContentPart::InputAudio(part) => {
            let filename = match part.input_audio.format {
                ct::ChatCompletionInputAudioFormat::Wav => Some("audio.wav".to_string()),
                ct::ChatCompletionInputAudioFormat::Mp3 => Some("audio.mp3".to_string()),
            };
            ot::ResponseInputContent::File(ot::ResponseInputFile {
                type_: ot::ResponseInputFileType::InputFile,
                detail: None,
                file_data: Some(part.input_audio.data),
                file_id: None,
                file_url: None,
                filename,
            })
        }
        ct::ChatCompletionContentPart::File(part) => {
            ot::ResponseInputContent::File(ot::ResponseInputFile {
                type_: ot::ResponseInputFileType::InputFile,
                detail: None,
                file_data: part.file.file_data,
                file_id: part.file.file_id,
                file_url: part.file.file_url,
                filename: part.file.filename,
            })
        }
    }
}

pub fn chat_text_content_to_response_input_message_content(
    content: ct::ChatCompletionTextContent,
) -> ot::ResponseInputMessageContent {
    match content {
        ct::ChatCompletionTextContent::Text(text) => ot::ResponseInputMessageContent::Text(text),
        ct::ChatCompletionTextContent::Parts(parts) => ot::ResponseInputMessageContent::List(
            parts
                .into_iter()
                .map(|part| {
                    ot::ResponseInputContent::Text(ot::ResponseInputText {
                        text: part.text,
                        type_: ot::ResponseInputTextType::InputText,
                    })
                })
                .collect::<Vec<_>>(),
        ),
    }
}

pub fn chat_text_content_to_plain_text(content: &ct::ChatCompletionTextContent) -> String {
    match content {
        ct::ChatCompletionTextContent::Text(text) => text.clone(),
        ct::ChatCompletionTextContent::Parts(parts) => parts
            .iter()
            .map(|part| part.text.clone())
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn chat_user_content_to_response_input_message_content(
    content: ct::ChatCompletionUserContent,
) -> ot::ResponseInputMessageContent {
    match content {
        ct::ChatCompletionUserContent::Text(text) => ot::ResponseInputMessageContent::Text(text),
        ct::ChatCompletionUserContent::Parts(parts) => ot::ResponseInputMessageContent::List(
            parts
                .into_iter()
                .map(chat_content_part_to_response_input_content)
                .collect::<Vec<_>>(),
        ),
    }
}

fn chat_custom_tool_format_to_response(
    format: Option<ct::ChatCompletionCustomToolFormat>,
) -> Option<ot::ResponseCustomToolInputFormat> {
    match format {
        Some(ct::ChatCompletionCustomToolFormat::Text(_)) => Some(
            ot::ResponseCustomToolInputFormat::Text(ot::ResponseCustomToolTextFormat {
                type_: ot::ResponseCustomToolTextFormatType::Text,
            }),
        ),
        Some(ct::ChatCompletionCustomToolFormat::Grammar(grammar)) => Some(
            ot::ResponseCustomToolInputFormat::Grammar(ot::ResponseCustomToolGrammarFormat {
                definition: grammar.grammar.definition,
                syntax: match grammar.grammar.syntax {
                    ct::ChatCompletionCustomToolGrammarSyntax::Lark => {
                        ot::ResponseCustomToolGrammarSyntax::Lark
                    }
                    ct::ChatCompletionCustomToolGrammarSyntax::Regex => {
                        ot::ResponseCustomToolGrammarSyntax::Regex
                    }
                },
                type_: ot::ResponseCustomToolGrammarFormatType::Grammar,
            }),
        ),
        None => None,
    }
}

pub fn chat_tools_to_response_tools(
    tools: Option<Vec<ct::ChatCompletionTool>>,
    functions: Option<Vec<ct::ChatCompletionLegacyFunction>>,
    web_search_options: Option<ct::ChatCompletionWebSearchOptions>,
) -> Option<Vec<ot::ResponseTool>> {
    let mut output = Vec::new();

    if let Some(tools) = tools {
        for tool in tools {
            match tool {
                ct::ChatCompletionTool::Function(tool) => {
                    output.push(ot::ResponseTool::Function(ot::ResponseFunctionTool {
                        name: tool.function.name,
                        parameters: tool.function.parameters.unwrap_or_default(),
                        strict: tool.function.strict,
                        type_: ot::ResponseFunctionToolType::Function,
                        defer_loading: None,
                        description: tool.function.description,
                    }));
                }
                ct::ChatCompletionTool::Custom(tool) => {
                    output.push(ot::ResponseTool::Custom(ot::ResponseCustomTool {
                        name: tool.custom.name,
                        type_: ot::ResponseCustomToolType::Custom,
                        defer_loading: None,
                        description: tool.custom.description,
                        format: chat_custom_tool_format_to_response(tool.custom.format),
                    }));
                }
            }
        }
    }

    if let Some(functions) = functions {
        for function in functions {
            output.push(ot::ResponseTool::Function(ot::ResponseFunctionTool {
                name: function.name,
                parameters: function.parameters.unwrap_or_default(),
                strict: None,
                type_: ot::ResponseFunctionToolType::Function,
                defer_loading: None,
                description: function.description,
            }));
        }
    }

    if let Some(web_search) = web_search_options {
        output.push(ot::ResponseTool::WebSearchPreview(
            ot::ResponseWebSearchPreviewTool {
                type_: ot::ResponseWebSearchPreviewToolType::WebSearchPreview,
                search_content_types: None,
                search_context_size: web_search.search_context_size.map(|size| match size {
                    ct::ChatCompletionWebSearchContextSize::Low => {
                        ot::ResponseWebSearchContextSize::Low
                    }
                    ct::ChatCompletionWebSearchContextSize::Medium => {
                        ot::ResponseWebSearchContextSize::Medium
                    }
                    ct::ChatCompletionWebSearchContextSize::High => {
                        ot::ResponseWebSearchContextSize::High
                    }
                }),
                user_location: web_search.user_location.map(|location| {
                    ot::ResponseWebSearchPreviewUserLocation {
                        type_: ot::ResponseApproximateLocationType::Approximate,
                        city: location.approximate.city,
                        country: location.approximate.country,
                        region: location.approximate.region,
                        timezone: location.approximate.timezone,
                    }
                }),
            },
        ));
    }

    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}

pub fn chat_reasoning_to_response_reasoning(
    reasoning_effort: Option<ct::ChatCompletionReasoningEffort>,
) -> Option<ot::ResponseReasoning> {
    reasoning_effort.map(|effort| ot::ResponseReasoning {
        effort: Some(match effort {
            ct::ChatCompletionReasoningEffort::None => ot::ResponseReasoningEffort::None,
            ct::ChatCompletionReasoningEffort::Minimal => ot::ResponseReasoningEffort::Minimal,
            ct::ChatCompletionReasoningEffort::Low => ot::ResponseReasoningEffort::Low,
            ct::ChatCompletionReasoningEffort::Medium => ot::ResponseReasoningEffort::Medium,
            ct::ChatCompletionReasoningEffort::High => ot::ResponseReasoningEffort::High,
            ct::ChatCompletionReasoningEffort::XHigh => ot::ResponseReasoningEffort::XHigh,
        }),
        generate_summary: None,
        summary: None,
    })
}

pub fn pseudo_reasoning_signature(message_index: usize, reasoning_ordinal: usize) -> String {
    format!("rs_{message_index}_{reasoning_ordinal}")
}

pub fn chat_response_text_config(
    response_format: Option<ct::ChatCompletionResponseFormat>,
    verbosity: Option<ct::ChatCompletionVerbosity>,
) -> Option<ot::ResponseTextConfig> {
    let format = response_format.map(|value| match value {
        ct::ChatCompletionResponseFormat::Text(_) => {
            ot::ResponseTextFormatConfig::Text(ot::ResponseFormatText {
                type_: ot::ResponseFormatTextType::Text,
            })
        }
        ct::ChatCompletionResponseFormat::JsonObject(_) => {
            ot::ResponseTextFormatConfig::JsonObject(ot::ResponseFormatJsonObject {
                type_: ot::ResponseFormatJsonObjectType::JsonObject,
            })
        }
        ct::ChatCompletionResponseFormat::JsonSchema(schema) => {
            ot::ResponseTextFormatConfig::JsonSchema(ot::ResponseFormatTextJsonSchemaConfig {
                name: schema.json_schema.name,
                schema: schema.json_schema.schema.unwrap_or_else(|| {
                    serde_json::from_str::<ot::JsonObject>(r#"{"type":"object"}"#)
                        .unwrap_or_default()
                }),
                type_: ot::ResponseFormatTextJsonSchemaConfigType::JsonSchema,
                description: schema.json_schema.description,
                strict: schema.json_schema.strict,
            })
        }
    });

    let verbosity = verbosity.map(|value| match value {
        ct::ChatCompletionVerbosity::Low => ot::ResponseTextVerbosity::Low,
        ct::ChatCompletionVerbosity::Medium => ot::ResponseTextVerbosity::Medium,
        ct::ChatCompletionVerbosity::High => ot::ResponseTextVerbosity::High,
    });

    if format.is_none() && verbosity.is_none() {
        None
    } else {
        Some(ot::ResponseTextConfig { format, verbosity })
    }
}

pub fn chat_tool_choice_to_response_tool_choice(
    tool_choice: Option<ct::ChatCompletionToolChoiceOption>,
    function_call: Option<ct::ChatCompletionFunctionCallOptionParam>,
) -> Option<ot::ResponseToolChoice> {
    if let Some(tool_choice) = tool_choice {
        return Some(match tool_choice {
            ct::ChatCompletionToolChoiceOption::Mode(mode) => {
                ot::ResponseToolChoice::Options(match mode {
                    ct::ChatCompletionToolChoiceMode::None => ot::ResponseToolChoiceOptions::None,
                    ct::ChatCompletionToolChoiceMode::Auto => ot::ResponseToolChoiceOptions::Auto,
                    ct::ChatCompletionToolChoiceMode::Required => {
                        ot::ResponseToolChoiceOptions::Required
                    }
                })
            }
            ct::ChatCompletionToolChoiceOption::Allowed(allowed) => {
                ot::ResponseToolChoice::Allowed(ot::ResponseToolChoiceAllowed {
                    mode: match allowed.allowed_tools.mode {
                        ct::ChatCompletionAllowedToolsMode::Auto => {
                            ot::ResponseToolChoiceAllowedMode::Auto
                        }
                        ct::ChatCompletionAllowedToolsMode::Required => {
                            ot::ResponseToolChoiceAllowedMode::Required
                        }
                    },
                    tools: allowed.allowed_tools.tools,
                    type_: ot::ResponseToolChoiceAllowedType::AllowedTools,
                })
            }
            ct::ChatCompletionToolChoiceOption::NamedFunction(tool) => {
                ot::ResponseToolChoice::Function(ot::ResponseToolChoiceFunction {
                    name: tool.function.name,
                    type_: ot::ResponseToolChoiceFunctionType::Function,
                })
            }
            ct::ChatCompletionToolChoiceOption::NamedCustom(tool) => {
                ot::ResponseToolChoice::Custom(ot::ResponseToolChoiceCustom {
                    name: tool.custom.name,
                    type_: ot::ResponseToolChoiceCustomType::Custom,
                })
            }
        });
    }

    function_call.map(|function_call| match function_call {
        ct::ChatCompletionFunctionCallOptionParam::Mode(mode) => {
            ot::ResponseToolChoice::Options(match mode {
                ct::ChatCompletionFunctionCallMode::None => ot::ResponseToolChoiceOptions::None,
                ct::ChatCompletionFunctionCallMode::Auto => ot::ResponseToolChoiceOptions::Auto,
            })
        }
        ct::ChatCompletionFunctionCallOptionParam::Named(named) => {
            ot::ResponseToolChoice::Function(ot::ResponseToolChoiceFunction {
                name: named.name,
                type_: ot::ResponseToolChoiceFunctionType::Function,
            })
        }
    })
}

pub fn chat_stop_to_vec(stop: Option<ct::ChatCompletionStop>) -> Option<Vec<String>> {
    match stop {
        Some(ct::ChatCompletionStop::Single(stop)) => Some(vec![stop]),
        Some(ct::ChatCompletionStop::Multiple(stops)) => Some(stops),
        None => None,
    }
}
