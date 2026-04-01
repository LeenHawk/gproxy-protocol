use crate::claude::count_tokens::request::ClaudeCountTokensRequest;
use crate::claude::count_tokens::types::HttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeCountTokensRequest> for ClaudeCountTokensRequest {
    type Error = TransformError;

    fn try_from(value: &ClaudeCountTokensRequest) -> Result<Self, TransformError> {
        let mut output = value.clone();
        output.method = HttpMethod::Post;
        Ok(output)
    }
}
