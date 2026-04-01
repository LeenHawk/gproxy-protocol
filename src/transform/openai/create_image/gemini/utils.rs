#![allow(dead_code)]
use crate::gemini::count_tokens::types as gt;
use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;
use crate::gemini::generate_content::types as gct;
use crate::gemini::types::GeminiResponseHeaders;
use crate::openai::create_image::types as it;
use crate::openai::create_image_edit::types as iet;
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::utils::TransformError;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GeminiGeneratedImageOutput {
    pub image: it::OpenAiGeneratedImage,
    pub output_format: Option<it::OpenAiImageOutputFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GeminiInlineImageOutput {
    pub candidate_index: u32,
    pub part_index: usize,
    pub b64_json: String,
    pub output_format: it::OpenAiImageOutputFormat,
}

fn gemini_image_config(aspect_ratio: &str) -> gt::GeminiImageConfig {
    gt::GeminiImageConfig {
        aspect_ratio: Some(aspect_ratio.to_string()),
        image_size: Some("1K".to_string()),
    }
}

pub(crate) fn gemini_image_config_from_create_image_size(
    size: Option<it::OpenAiImageSize>,
) -> Result<Option<gt::GeminiImageConfig>, TransformError> {
    match size {
        Some(it::OpenAiImageSize::Auto) | None => Ok(None),
        Some(it::OpenAiImageSize::S1024x1024) => Ok(Some(gemini_image_config("1:1"))),
        Some(it::OpenAiImageSize::S1536x1024) => Ok(Some(gemini_image_config("3:2"))),
        Some(it::OpenAiImageSize::S1024x1536) => Ok(Some(gemini_image_config("2:3"))),
        Some(
            it::OpenAiImageSize::S256x256
            | it::OpenAiImageSize::S512x512
            | it::OpenAiImageSize::S1792x1024
            | it::OpenAiImageSize::S1024x1792,
        ) => Err(TransformError::not_implemented(
            "cannot convert OpenAI image request with unsupported size to Gemini generateContent request",
        )),
    }
}

pub(crate) fn gemini_image_config_from_create_image_edit_size(
    size: Option<iet::OpenAiImageEditSize>,
) -> Option<gt::GeminiImageConfig> {
    match size {
        Some(iet::OpenAiImageEditSize::Auto) | None => None,
        Some(iet::OpenAiImageEditSize::S1024x1024) => Some(gemini_image_config("1:1")),
        Some(iet::OpenAiImageEditSize::S1536x1024) => Some(gemini_image_config("3:2")),
        Some(iet::OpenAiImageEditSize::S1024x1536) => Some(gemini_image_config("2:3")),
    }
}

fn parse_base64_data_url(value: &str) -> Result<gt::GeminiBlob, TransformError> {
    let payload = value.strip_prefix("data:").ok_or(TransformError::not_implemented(
        "cannot convert OpenAI image edit request with invalid data URL input image to Gemini generateContent request",
    ))?;
    let (metadata, data) = payload.split_once(',').ok_or(TransformError::not_implemented(
        "cannot convert OpenAI image edit request with invalid data URL input image to Gemini generateContent request",
    ))?;
    let mime_type = metadata
        .strip_suffix(";base64")
        .ok_or(TransformError::not_implemented(
            "cannot convert OpenAI image edit request with invalid data URL input image to Gemini generateContent request",
        ))?;

    if mime_type.is_empty() || data.is_empty() {
        return Err(TransformError::not_implemented(
            "cannot convert OpenAI image edit request with invalid data URL input image to Gemini generateContent request",
        ));
    }

    Ok(gt::GeminiBlob {
        mime_type: mime_type.to_string(),
        data: data.to_string(),
    })
}

pub(crate) fn gemini_part_from_openai_edit_input_image(
    image: iet::OpenAiImageEditInputImage,
) -> Result<gt::GeminiPart, TransformError> {
    if image.file_id.is_some() {
        return Err(TransformError::not_implemented(
            "cannot convert OpenAI image edit request with file_id input image to Gemini generateContent request",
        ));
    }

    let image_url = image.image_url.ok_or(TransformError::not_implemented(
        "cannot convert OpenAI image edit request without image_url input image to Gemini generateContent request",
    ))?;

    if image_url.is_empty() {
        return Err(TransformError::not_implemented(
            "cannot convert OpenAI image edit request without image_url input image to Gemini generateContent request",
        ));
    }

    if image_url.starts_with("data:") {
        return Ok(gt::GeminiPart {
            inline_data: Some(parse_base64_data_url(&image_url)?),
            ..gt::GeminiPart::default()
        });
    }

    Ok(gt::GeminiPart {
        file_data: Some(gt::GeminiFileData {
            mime_type: None,
            file_uri: image_url,
        }),
        ..gt::GeminiPart::default()
    })
}

pub(crate) fn openai_response_headers_from_gemini(
    headers: GeminiResponseHeaders,
) -> OpenAiResponseHeaders {
    OpenAiResponseHeaders {
        extra: headers.extra,
    }
}

pub(crate) fn openai_output_format_from_mime(
    mime_type: &str,
) -> Option<it::OpenAiImageOutputFormat> {
    match mime_type.to_ascii_lowercase().as_str() {
        "image/png" => Some(it::OpenAiImageOutputFormat::Png),
        "image/jpeg" | "image/jpg" => Some(it::OpenAiImageOutputFormat::Jpeg),
        "image/webp" => Some(it::OpenAiImageOutputFormat::Webp),
        _ => None,
    }
}

pub(crate) fn gemini_generated_image_outputs_from_response(
    body: &GeminiGenerateContentResponseBody,
) -> Vec<GeminiGeneratedImageOutput> {
    let mut outputs = Vec::new();

    let Some(candidates) = body.candidates.as_ref() else {
        return outputs;
    };

    for candidate in candidates {
        let Some(content) = candidate.content.as_ref() else {
            continue;
        };

        for part in &content.parts {
            if let Some(inline_data) = part.inline_data.as_ref()
                && inline_data.mime_type.starts_with("image/")
                && !inline_data.data.is_empty()
            {
                outputs.push(GeminiGeneratedImageOutput {
                    image: it::OpenAiGeneratedImage {
                        b64_json: Some(inline_data.data.clone()),
                        revised_prompt: None,
                        url: None,
                    },
                    output_format: openai_output_format_from_mime(&inline_data.mime_type),
                });
            }

            if let Some(file_data) = part.file_data.as_ref()
                && !file_data.file_uri.is_empty()
                && file_data
                    .mime_type
                    .as_deref()
                    .is_none_or(|mime_type| mime_type.starts_with("image/"))
            {
                outputs.push(GeminiGeneratedImageOutput {
                    image: it::OpenAiGeneratedImage {
                        b64_json: None,
                        revised_prompt: None,
                        url: Some(file_data.file_uri.clone()),
                    },
                    output_format: file_data
                        .mime_type
                        .as_deref()
                        .and_then(openai_output_format_from_mime),
                });
            }
        }
    }

    outputs
}

pub(crate) fn gemini_inline_image_outputs_from_response(
    body: &GeminiGenerateContentResponseBody,
) -> Vec<GeminiInlineImageOutput> {
    let mut outputs = Vec::new();

    let Some(candidates) = body.candidates.as_ref() else {
        return outputs;
    };

    for (candidate_pos, candidate) in candidates.iter().enumerate() {
        let Some(content) = candidate.content.as_ref() else {
            continue;
        };
        let candidate_index = candidate.index.unwrap_or(candidate_pos as u32);

        for (part_index, part) in content.parts.iter().enumerate() {
            let Some(inline_data) = part.inline_data.as_ref() else {
                continue;
            };
            if !inline_data.mime_type.starts_with("image/") || inline_data.data.is_empty() {
                continue;
            }
            let Some(output_format) = openai_output_format_from_mime(&inline_data.mime_type) else {
                continue;
            };
            outputs.push(GeminiInlineImageOutput {
                candidate_index,
                part_index,
                b64_json: inline_data.data.clone(),
                output_format,
            });
        }
    }

    outputs
}

fn modality_token_count(
    details: Option<&Vec<gt::GeminiModalityTokenCount>>,
    modality: gt::GeminiModality,
) -> u64 {
    details
        .into_iter()
        .flat_map(|details| details.iter())
        .filter(|detail| detail.modality == modality)
        .map(|detail| detail.token_count)
        .sum()
}

pub(crate) fn openai_image_usage_from_gemini(
    usage: Option<&gct::GeminiUsageMetadata>,
) -> Option<it::OpenAiImageUsage> {
    let usage = usage?;

    let input_details = it::OpenAiImageTokenDetails {
        image_tokens: modality_token_count(
            usage.prompt_tokens_details.as_ref(),
            gt::GeminiModality::Image,
        ),
        text_tokens: modality_token_count(
            usage.prompt_tokens_details.as_ref(),
            gt::GeminiModality::Text,
        ),
    };
    let output_details = it::OpenAiImageTokenDetails {
        image_tokens: modality_token_count(
            usage.candidates_tokens_details.as_ref(),
            gt::GeminiModality::Image,
        ),
        text_tokens: modality_token_count(
            usage.candidates_tokens_details.as_ref(),
            gt::GeminiModality::Text,
        ),
    };

    let input_tokens = usage
        .prompt_token_count
        .unwrap_or(input_details.image_tokens + input_details.text_tokens);
    let output_tokens = usage
        .candidates_token_count
        .unwrap_or(output_details.image_tokens + output_details.text_tokens);

    Some(it::OpenAiImageUsage {
        input_tokens,
        input_tokens_details: input_details,
        output_tokens,
        total_tokens: usage
            .total_token_count
            .unwrap_or(input_tokens.saturating_add(output_tokens)),
        output_tokens_details: usage
            .candidates_tokens_details
            .as_ref()
            .map(|_| output_details),
    })
}

pub(crate) fn best_effort_openai_image_usage_from_gemini(
    usage: Option<&gct::GeminiUsageMetadata>,
) -> it::OpenAiImageUsage {
    openai_image_usage_from_gemini(usage).unwrap_or(it::OpenAiImageUsage {
        input_tokens: 0,
        input_tokens_details: it::OpenAiImageTokenDetails {
            image_tokens: 0,
            text_tokens: 0,
        },
        output_tokens: 0,
        total_tokens: 0,
        output_tokens_details: None,
    })
}

pub(crate) fn create_image_response_body_from_gemini_response(
    body: GeminiGenerateContentResponseBody,
) -> Result<it::OpenAiCreateImageResponseBody, TransformError> {
    let outputs = gemini_generated_image_outputs_from_response(&body);
    if outputs.is_empty() {
        return Err(TransformError::not_implemented(
            "cannot convert Gemini generateContent response without image output to OpenAI create image response",
        ));
    }

    let mut data = Vec::with_capacity(outputs.len());
    let mut common_output_format: Option<Option<it::OpenAiImageOutputFormat>> = None;
    let mut same_output_format = true;

    for output in outputs {
        if let Some(existing) = common_output_format.as_ref() {
            if *existing != output.output_format {
                same_output_format = false;
            }
        } else {
            common_output_format = Some(output.output_format.clone());
        }
        data.push(output.image);
    }

    Ok(it::OpenAiCreateImageResponseBody {
        created: 0,
        background: None,
        data: Some(data),
        output_format: if same_output_format {
            common_output_format.flatten()
        } else {
            None
        },
        quality: None,
        size: None,
        usage: openai_image_usage_from_gemini(body.usage_metadata.as_ref()),
    })
}
