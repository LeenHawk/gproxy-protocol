use crate::claude::create_message::request::ClaudeCreateMessageRequest;
use crate::claude::create_message::types::HttpMethod;
use crate::transform::utils::TransformError;

pub fn normalize_claude_stream_request(
    value: &ClaudeCreateMessageRequest,
) -> Result<ClaudeCreateMessageRequest, TransformError> {
    let mut output = value.clone();
    output.method = HttpMethod::Post;
    output.body.stream = Some(true);
    Ok(output)
}
