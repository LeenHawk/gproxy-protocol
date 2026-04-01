use crate::claude::count_tokens::response::ClaudeCountTokensResponse;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeCountTokensResponse> for ClaudeCountTokensResponse {
    type Error = TransformError;

    fn try_from(value: &ClaudeCountTokensResponse) -> Result<Self, TransformError> {
        Ok(value.clone())
    }
}
