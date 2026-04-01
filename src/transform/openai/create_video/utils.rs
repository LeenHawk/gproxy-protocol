use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::gemini::count_tokens::types as gt;
use crate::gemini::generate_videos::response::ResponseBody as GeminiGenerateVideosResponseBody;
use crate::gemini::generate_videos::types as gvt;
use crate::openai::create_video::types::{
    OpenAiVideo, OpenAiVideoCreateError, OpenAiVideoModel, OpenAiVideoObject, OpenAiVideoSize,
    OpenAiVideoStatus,
};
use crate::transform::gemini::model_get::utils::strip_models_prefix;
use crate::transform::utils::TransformError;

const GEMINI_VIDEO_ID_PREFIX: &str = "vid_gemini_";
const DEFAULT_GEMINI_VIDEO_MODEL: &str = "veo-3.1-generate-preview";
const DEFAULT_GEMINI_VIDEO_SECONDS: &str = "8";

pub(crate) fn openai_video_model_to_string(model: &OpenAiVideoModel) -> String {
    serde_json::to_value(model)
        .ok()
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

pub(crate) fn encode_gemini_video_operation_id(operation: &str) -> String {
    format!(
        "{GEMINI_VIDEO_ID_PREFIX}{}",
        URL_SAFE_NO_PAD.encode(operation.as_bytes())
    )
}

pub(crate) fn decode_gemini_video_operation_id(video_id: &str) -> Result<String, TransformError> {
    let encoded = video_id.strip_prefix(GEMINI_VIDEO_ID_PREFIX).ok_or(
        TransformError::not_implemented(
            "cannot convert OpenAI video request with non-Gemini video id to Gemini Veo request",
        ),
    )?;

    let decoded = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| {
            TransformError::not_implemented(
                "cannot convert OpenAI video request with invalid Gemini video id to Gemini Veo request",
            )
        })?;

    String::from_utf8(decoded).map_err(|_| {
        TransformError::not_implemented(
            "cannot convert OpenAI video request with invalid Gemini video id to Gemini Veo request",
        )
    })
}

pub(crate) fn parse_base64_data_url(value: &str) -> Result<gt::GeminiBlob, TransformError> {
    let payload = value.strip_prefix("data:").ok_or(TransformError::not_implemented(
        "cannot convert OpenAI video request with invalid data URL reference image to Gemini Veo request",
    ))?;
    let (metadata, data) = payload.split_once(',').ok_or(TransformError::not_implemented(
        "cannot convert OpenAI video request with invalid data URL reference image to Gemini Veo request",
    ))?;
    let mime_type = metadata
        .strip_suffix(";base64")
        .ok_or(TransformError::not_implemented(
            "cannot convert OpenAI video request with invalid data URL reference image to Gemini Veo request",
        ))?;

    if mime_type.is_empty() || data.is_empty() {
        return Err(TransformError::not_implemented(
            "cannot convert OpenAI video request with invalid data URL reference image to Gemini Veo request",
        ));
    }

    Ok(gt::GeminiBlob {
        mime_type: mime_type.to_string(),
        data: data.to_string(),
    })
}

pub(crate) fn gemini_video_asset_from_image_url(
    image_url: &str,
) -> Result<gvt::GeminiVideoGenerationAsset, TransformError> {
    if image_url.is_empty() {
        return Err(TransformError::not_implemented(
            "cannot convert OpenAI video request with empty image_url reference image to Gemini Veo request",
        ));
    }

    if image_url.starts_with("data:") {
        return Ok(gvt::GeminiVideoGenerationAsset {
            inline_data: Some(parse_base64_data_url(image_url)?),
            file_data: None,
        });
    }

    Ok(gvt::GeminiVideoGenerationAsset {
        inline_data: None,
        file_data: Some(gt::GeminiFileData {
            mime_type: None,
            file_uri: image_url.to_string(),
        }),
    })
}

pub(crate) fn gemini_aspect_ratio_from_openai_size(size: OpenAiVideoSize) -> &'static str {
    match size {
        OpenAiVideoSize::S720x1280 | OpenAiVideoSize::S1024x1792 => "9:16",
        OpenAiVideoSize::S1280x720 | OpenAiVideoSize::S1792x1024 => "16:9",
    }
}

fn timestamp_from_rfc3339(value: Option<&str>) -> Option<u64> {
    let value = value?.trim();
    if value.is_empty() {
        return None;
    }
    OffsetDateTime::parse(value, &Rfc3339)
        .ok()
        .and_then(|value| u64::try_from(value.unix_timestamp()).ok())
}

fn progress_percent(metadata: Option<&gvt::GeminiVideoOperationMetadata>, done: bool) -> f64 {
    if done {
        return 100.0;
    }

    metadata
        .and_then(|value| value.progress_percent.or(value.progress))
        .unwrap_or(0.0)
        .clamp(0.0, 100.0)
}

fn openai_video_size_from_aspect_ratio(aspect_ratio: Option<&str>) -> OpenAiVideoSize {
    match aspect_ratio.map(str::trim) {
        Some("9:16") => OpenAiVideoSize::S720x1280,
        _ => OpenAiVideoSize::S1280x720,
    }
}

fn openai_video_seconds_from_metadata(
    metadata: Option<&gvt::GeminiVideoOperationMetadata>,
) -> String {
    metadata
        .and_then(|value| value.duration_seconds.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_GEMINI_VIDEO_SECONDS)
        .to_string()
}

fn openai_video_model_from_metadata(
    metadata: Option<&gvt::GeminiVideoOperationMetadata>,
) -> OpenAiVideoModel {
    let model = metadata
        .and_then(|value| value.model.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(strip_models_prefix)
        .unwrap_or_else(|| DEFAULT_GEMINI_VIDEO_MODEL.to_string());
    OpenAiVideoModel::Custom(model)
}

fn openai_video_error_from_operation(
    error: Option<&gvt::GeminiApiError>,
) -> Option<OpenAiVideoCreateError> {
    let error = error?;
    Some(OpenAiVideoCreateError {
        code: error
            .status
            .clone()
            .unwrap_or_else(|| error.code.to_string())
            .to_lowercase(),
        message: error.message.clone(),
    })
}

fn openai_video_status_from_operation(
    body: &GeminiGenerateVideosResponseBody,
) -> OpenAiVideoStatus {
    if body.error.is_some() {
        return OpenAiVideoStatus::Failed;
    }

    if body.done.unwrap_or(false) {
        return OpenAiVideoStatus::Completed;
    }

    let progress = progress_percent(body.metadata.as_ref(), false);
    if progress > 0.0 {
        OpenAiVideoStatus::InProgress
    } else {
        OpenAiVideoStatus::Queued
    }
}

#[allow(dead_code)]
pub(crate) fn gemini_video_download_uri(body: &GeminiGenerateVideosResponseBody) -> Option<String> {
    body.response
        .as_ref()
        .and_then(|response| {
            response
                .generate_video_response
                .as_ref()
                .and_then(|value| value.generated_samples.as_ref())
                .or(response.generated_videos.as_ref())
        })
        .into_iter()
        .flat_map(|samples| samples.iter())
        .find_map(|sample| {
            sample
                .video
                .as_ref()
                .and_then(|video| video.uri.as_deref())
                .map(ToOwned::to_owned)
        })
}

pub(crate) fn openai_video_from_gemini_operation(
    body: GeminiGenerateVideosResponseBody,
) -> OpenAiVideo {
    let metadata = body.metadata.as_ref();
    let done = body.done.unwrap_or(false);
    let status = openai_video_status_from_operation(&body);

    OpenAiVideo {
        id: encode_gemini_video_operation_id(&body.name),
        completed_at: if done {
            timestamp_from_rfc3339(
                metadata
                    .and_then(|value| value.end_time.as_deref())
                    .or(metadata.and_then(|value| value.update_time.as_deref())),
            )
        } else {
            None
        },
        created_at: timestamp_from_rfc3339(metadata.and_then(|value| value.create_time.as_deref()))
            .unwrap_or(0),
        error: openai_video_error_from_operation(body.error.as_ref()),
        expires_at: None,
        model: openai_video_model_from_metadata(metadata),
        object: OpenAiVideoObject::Video,
        progress: progress_percent(metadata, done),
        prompt: metadata
            .and_then(|value| value.prompt.clone())
            .unwrap_or_default(),
        remixed_from_video_id: None,
        seconds: openai_video_seconds_from_metadata(metadata),
        size: openai_video_size_from_aspect_ratio(
            metadata.and_then(|value| value.aspect_ratio.as_deref()),
        ),
        status,
    }
}
