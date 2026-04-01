use http::StatusCode;

use crate::openai::create_response::response::OpenAiCreateResponseResponse;
use crate::openai::create_response::stream::{ResponseStreamErrorPayload, ResponseStreamEvent};
use crate::openai::create_response::types::{OpenAiApiError, OpenAiApiErrorResponse};
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::utils::TransformError;

impl TryFrom<Vec<ResponseStreamEvent>> for OpenAiCreateResponseResponse {
    type Error = TransformError;

    fn try_from(value: Vec<ResponseStreamEvent>) -> Result<Self, TransformError> {
        let mut latest_response = None;
        let mut stream_error = None::<ResponseStreamErrorPayload>;

        for event in value {
            match event {
                ResponseStreamEvent::Created { response, .. }
                | ResponseStreamEvent::Queued { response, .. }
                | ResponseStreamEvent::InProgress { response, .. }
                | ResponseStreamEvent::Completed { response, .. }
                | ResponseStreamEvent::Incomplete { response, .. }
                | ResponseStreamEvent::Failed { response, .. } => {
                    latest_response = Some(response);
                }
                ResponseStreamEvent::Error { error, .. } => stream_error = Some(error),
                _ => {}
            }
        }

        if let Some(body) = latest_response {
            Ok(OpenAiCreateResponseResponse::Success {
                stats_code: StatusCode::OK,
                headers: OpenAiResponseHeaders::default(),
                body,
            })
        } else if let Some(error) = stream_error {
            Ok(OpenAiCreateResponseResponse::Error {
                stats_code: StatusCode::BAD_REQUEST,
                headers: OpenAiResponseHeaders::default(),
                body: OpenAiApiErrorResponse {
                    error: OpenAiApiError {
                        message: error.message,
                        type_: error.type_,
                        param: error.param,
                        code: error.code,
                    },
                },
            })
        } else {
            Err(TransformError::not_implemented(
                "cannot convert OpenAI response SSE stream body without response snapshots",
            ))
        }
    }
}
