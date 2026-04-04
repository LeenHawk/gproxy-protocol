use crate::openai::create_image::response::OpenAiCreateImageResponse;
use crate::openai::create_response::response::OpenAiCreateResponseResponse;
use crate::transform::openai::create_image::utils::{
    PreferredImageAction, create_image_response_body_from_response,
};
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiCreateResponseResponse> for OpenAiCreateImageResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiCreateResponseResponse) -> Result<Self, TransformError> {
        match value {
            OpenAiCreateResponseResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let image_body =
                    create_image_response_body_from_response(body, PreferredImageAction::Generate)?;
                Ok(OpenAiCreateImageResponse::Success {
                    stats_code,
                    headers,
                    body: image_body,
                })
            }
            OpenAiCreateResponseResponse::Error {
                stats_code,
                headers,
                body,
            } => Ok(OpenAiCreateImageResponse::Error {
                stats_code,
                headers,
                body,
            }),
        }
    }
}
