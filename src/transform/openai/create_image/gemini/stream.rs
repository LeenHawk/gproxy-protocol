use crate::openai::create_image::stream::{
    ImageGenerationStreamEvent, OpenAiCreateImageSseData, OpenAiCreateImageSseEvent,
};
use crate::transform::openai::create_image::utils::stream_error_from_response_error;

pub fn completed_image_event(
    b64_json: String,
    output_format: crate::openai::create_image::types::OpenAiImageOutputFormat,
    usage: crate::openai::create_image::types::OpenAiImageUsage,
) -> OpenAiCreateImageSseEvent {
    OpenAiCreateImageSseEvent {
        event: None,
        data: OpenAiCreateImageSseData::Event(ImageGenerationStreamEvent::Completed {
            b64_json,
            background: crate::openai::create_image::types::OpenAiImageBackground::Auto,
            created_at: 0,
            output_format,
            quality: crate::openai::create_image::types::OpenAiImageQuality::Auto,
            size: crate::openai::create_image::types::OpenAiImageSize::Auto,
            usage,
        }),
    }
}

pub fn error_event(code: String, message: String) -> OpenAiCreateImageSseEvent {
    OpenAiCreateImageSseEvent {
        event: None,
        data: OpenAiCreateImageSseData::Event(ImageGenerationStreamEvent::Error {
            error: stream_error_from_response_error(Some(code), message, None),
        }),
    }
}

pub fn done_event() -> OpenAiCreateImageSseEvent {
    OpenAiCreateImageSseEvent {
        event: None,
        data: OpenAiCreateImageSseData::Done("[DONE]".to_string()),
    }
}
