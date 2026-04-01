use crate::openai::create_image_edit::stream::{
    ImageEditStreamEvent, OpenAiCreateImageEditSseData, OpenAiCreateImageEditSseEvent,
};
use crate::transform::openai::create_image::utils::stream_error_from_response_error;

pub fn completed_image_event(
    b64_json: String,
    output_format: crate::openai::create_image::types::OpenAiImageOutputFormat,
    usage: crate::openai::create_image::types::OpenAiImageUsage,
) -> OpenAiCreateImageEditSseEvent {
    OpenAiCreateImageEditSseEvent {
        event: None,
        data: OpenAiCreateImageEditSseData::Event(ImageEditStreamEvent::Completed {
            b64_json,
            background: crate::openai::create_image::types::OpenAiImageBackground::Auto,
            created_at: 0,
            output_format,
            quality: crate::openai::create_image_edit::types::OpenAiImageEditQuality::Auto,
            size: crate::openai::create_image_edit::types::OpenAiImageEditSize::Auto,
            usage,
        }),
    }
}

pub fn error_event(code: String, message: String) -> OpenAiCreateImageEditSseEvent {
    OpenAiCreateImageEditSseEvent {
        event: None,
        data: OpenAiCreateImageEditSseData::Event(ImageEditStreamEvent::Error {
            error: stream_error_from_response_error(Some(code), message, None),
        }),
    }
}

pub fn done_event() -> OpenAiCreateImageEditSseEvent {
    OpenAiCreateImageEditSseEvent {
        event: None,
        data: OpenAiCreateImageEditSseData::Done("[DONE]".to_string()),
    }
}
