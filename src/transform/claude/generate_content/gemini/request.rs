use crate::claude::count_tokens::types::{
    BetaContentBlockParam, BetaMessageContent, BetaMessageRole,
};
use crate::claude::create_message::request::ClaudeCreateMessageRequest;
use crate::gemini::count_tokens::types::{
    GeminiBlob, GeminiContentRole, GeminiFileData, GeminiFunctionCall, GeminiPart,
};
use crate::gemini::generate_content::request::{
    GeminiGenerateContentRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::generate_content::types::{GeminiContent, GeminiGenerationConfig, HttpMethod};
use crate::transform::claude::generate_content::gemini::utils::{
    gemini_system_instruction_from_claude, gemini_thinking_config_from_claude,
    gemini_tool_config_from_claude, gemini_tools_from_claude,
};
use crate::transform::claude::generate_content::utils::{
    beta_message_content_to_text, beta_mid_conversation_system_block_to_text,
    claude_model_to_string,
};
use crate::transform::claude::model_list::gemini::utils::ensure_models_prefix;
use crate::transform::utils::TransformError;

fn gemini_text_content(role: GeminiContentRole, text: String) -> Option<GeminiContent> {
    if text.is_empty() {
        return None;
    }

    Some(GeminiContent {
        parts: vec![GeminiPart {
            text: Some(text),
            ..GeminiPart::default()
        }],
        role: Some(role),
    })
}

fn flush_gemini_parts(
    contents: &mut Vec<GeminiContent>,
    role: GeminiContentRole,
    parts: &mut Vec<GeminiPart>,
) {
    if parts.is_empty() {
        return;
    }

    contents.push(GeminiContent {
        parts: std::mem::take(parts),
        role: Some(role),
    });
}

fn claude_blocks_to_gemini_contents(
    role: GeminiContentRole,
    blocks: Vec<BetaContentBlockParam>,
    fallback_text: String,
) -> Vec<GeminiContent> {
    let mut contents = Vec::new();
    let mut parts = Vec::new();

    for block in blocks {
        match block {
            BetaContentBlockParam::Text(block) => {
                parts.push(GeminiPart {
                    text: Some(block.text),
                    ..GeminiPart::default()
                });
            }
            BetaContentBlockParam::Thinking(block) => {
                parts.push(GeminiPart {
                    thought: Some(true),
                    thought_signature: Some(block.signature),
                    text: Some(block.thinking),
                    ..GeminiPart::default()
                });
            }
            BetaContentBlockParam::ToolUse(block) => {
                parts.push(GeminiPart {
                    function_call: Some(GeminiFunctionCall {
                        id: Some(block.id),
                        name: block.name,
                        args: Some(block.input),
                    }),
                    ..GeminiPart::default()
                });
            }
            BetaContentBlockParam::Image(block) => match block.source {
                crate::claude::count_tokens::types::BetaImageSource::Base64(source) => {
                    let mime_type = match source.media_type {
                        crate::claude::count_tokens::types::BetaImageMediaType::ImageJpeg => {
                            "image/jpeg"
                        }
                        crate::claude::count_tokens::types::BetaImageMediaType::ImagePng => {
                            "image/png"
                        }
                        crate::claude::count_tokens::types::BetaImageMediaType::ImageGif => {
                            "image/gif"
                        }
                        crate::claude::count_tokens::types::BetaImageMediaType::ImageWebp => {
                            "image/webp"
                        }
                    };
                    parts.push(GeminiPart {
                        inline_data: Some(GeminiBlob {
                            mime_type: mime_type.to_string(),
                            data: source.data,
                        }),
                        ..GeminiPart::default()
                    });
                }
                crate::claude::count_tokens::types::BetaImageSource::Url(source) => {
                    parts.push(GeminiPart {
                        file_data: Some(GeminiFileData {
                            mime_type: None,
                            file_uri: source.url,
                        }),
                        ..GeminiPart::default()
                    });
                }
                crate::claude::count_tokens::types::BetaImageSource::File(source) => {
                    parts.push(GeminiPart {
                        text: Some(format!("file_id:{}", source.file_id)),
                        ..GeminiPart::default()
                    });
                }
            },
            BetaContentBlockParam::MidConversationSystem(block) => {
                flush_gemini_parts(&mut contents, role.clone(), &mut parts);
                if let Some(content) = gemini_text_content(
                    GeminiContentRole::User,
                    beta_mid_conversation_system_block_to_text(&block),
                ) {
                    contents.push(content);
                }
            }
            _ => {}
        }
    }

    flush_gemini_parts(&mut contents, role.clone(), &mut parts);

    if contents.is_empty()
        && let Some(content) = gemini_text_content(role, fallback_text)
    {
        contents.push(content);
    }

    contents
}

impl TryFrom<ClaudeCreateMessageRequest> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: ClaudeCreateMessageRequest) -> Result<Self, TransformError> {
        let body = value.body;
        let model = ensure_models_prefix(&claude_model_to_string(&body.model));

        let contents = body
            .messages
            .into_iter()
            .flat_map(|message| {
                let fallback_text = beta_message_content_to_text(&message.content);
                let role = match message.role {
                    BetaMessageRole::User => GeminiContentRole::User,
                    BetaMessageRole::Assistant => GeminiContentRole::Model,
                    BetaMessageRole::System => GeminiContentRole::User,
                }
                .clone();

                match message.content {
                    BetaMessageContent::Text(text) => gemini_text_content(role, text)
                        .into_iter()
                        .collect::<Vec<_>>(),
                    BetaMessageContent::Blocks(blocks) => {
                        claude_blocks_to_gemini_contents(role, blocks, fallback_text)
                    }
                }
            })
            .collect::<Vec<_>>();
        let system_instruction = gemini_system_instruction_from_claude(body.system);
        let tools = gemini_tools_from_claude(body.tools, true);
        let tool_config = gemini_tool_config_from_claude(body.tool_choice);

        let mut generation_config = GeminiGenerationConfig::default();
        let mut has_generation_config = true;
        generation_config.max_output_tokens = Some(body.max_tokens.min(u32::MAX as u64) as u32);
        if let Some(stop_sequences) = body.stop_sequences {
            generation_config.stop_sequences = Some(stop_sequences);
            has_generation_config = true;
        }
        if let Some(temperature) = body.temperature {
            generation_config.temperature = Some(temperature);
            has_generation_config = true;
        }
        if let Some(top_p) = body.top_p {
            generation_config.top_p = Some(top_p);
            has_generation_config = true;
        }
        if let Some(top_k) = body.top_k {
            generation_config.top_k = Some(top_k.min(u32::MAX as u64) as u32);
            has_generation_config = true;
        }
        let thinking_config = gemini_thinking_config_from_claude(
            body.thinking,
            body.output_config
                .as_ref()
                .and_then(|config| config.effort.as_ref()),
        );
        if let Some(thinking_config) = thinking_config {
            generation_config.thinking_config = Some(thinking_config);
            has_generation_config = true;
        }
        let json_output_requested = body
            .output_config
            .as_ref()
            .and_then(|config| config.format.as_ref())
            .is_some();
        let response_json_schema = body
            .output_config
            .as_ref()
            .and_then(|config| config.format.as_ref())
            .and_then(|schema| serde_json::to_value(schema.schema.clone()).ok());
        if json_output_requested {
            generation_config.response_mime_type = Some("application/json".to_string());
            has_generation_config = true;
        }
        if let Some(schema) = response_json_schema {
            generation_config.response_json_schema = Some(schema);
            has_generation_config = true;
        }
        let generation_config = if has_generation_config {
            Some(generation_config)
        } else {
            None
        };

        Ok(Self {
            method: HttpMethod::Post,
            path: PathParameters { model },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                contents,
                tools,
                tool_config,
                safety_settings: None,
                system_instruction,
                generation_config,
                cached_content: None,
                store: None,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::count_tokens::types as ct;

    fn request_with_messages(messages: Vec<ct::BetaMessageParam>) -> ClaudeCreateMessageRequest {
        ClaudeCreateMessageRequest {
            method: crate::claude::create_message::types::HttpMethod::Post,
            path: crate::claude::create_message::request::PathParameters::default(),
            query: crate::claude::create_message::request::QueryParameters::default(),
            headers: crate::claude::create_message::request::RequestHeaders::default(),
            body: crate::claude::create_message::request::RequestBody {
                max_tokens: 1024,
                messages,
                model: ct::Model::Custom("claude-test".to_string()),
                container: None,
                context_management: None,
                inference_geo: None,
                mcp_servers: None,
                metadata: None,
                cache_control: None,
                output_config: None,
                service_tier: None,
                speed: None,
                stop_sequences: None,
                stream: None,
                system: None,
                temperature: None,
                thinking: None,
                tool_choice: None,
                tools: None,
                top_k: None,
                top_p: None,
            },
        }
    }

    fn content_text(content: &GeminiContent) -> String {
        content
            .parts
            .iter()
            .filter_map(|part| part.text.as_deref())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn mid_conversation_system_block_becomes_gemini_user_content() {
        let request = request_with_messages(vec![ct::BetaMessageParam {
            role: ct::BetaMessageRole::User,
            content: ct::BetaMessageContent::Blocks(vec![
                ct::BetaContentBlockParam::Text(ct::BetaTextBlockParam {
                    text: "First user turn".to_string(),
                    type_: ct::BetaTextBlockType::Text,
                    cache_control: None,
                    citations: None,
                }),
                ct::BetaContentBlockParam::MidConversationSystem(
                    ct::BetaMidConversationSystemBlockParam {
                        content: vec![ct::BetaTextBlockParam {
                            text: "Apply the new policy now.".to_string(),
                            type_: ct::BetaTextBlockType::Text,
                            cache_control: None,
                            citations: None,
                        }],
                        type_: ct::BetaMidConversationSystemBlockType::MidConvSystem,
                        cache_control: None,
                    },
                ),
                ct::BetaContentBlockParam::Text(ct::BetaTextBlockParam {
                    text: "Second user turn".to_string(),
                    type_: ct::BetaTextBlockType::Text,
                    cache_control: None,
                    citations: None,
                }),
            ]),
        }]);

        let converted = GeminiGenerateContentRequest::try_from(request).expect("request converts");
        assert_eq!(converted.body.contents.len(), 3);
        assert_eq!(
            converted.body.contents[0].role,
            Some(GeminiContentRole::User)
        );
        assert_eq!(content_text(&converted.body.contents[0]), "First user turn");
        assert_eq!(
            converted.body.contents[1].role,
            Some(GeminiContentRole::User)
        );
        assert_eq!(
            content_text(&converted.body.contents[1]),
            "Apply the new policy now."
        );
        assert_eq!(
            content_text(&converted.body.contents[2]),
            "Second user turn"
        );
    }
}
