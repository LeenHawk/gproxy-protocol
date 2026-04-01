use crate::claude::model_list::request::ClaudeModelListRequest;
use crate::claude::types::HttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeModelListRequest> for ClaudeModelListRequest {
    type Error = TransformError;

    fn try_from(value: &ClaudeModelListRequest) -> Result<Self, TransformError> {
        let mut output = value.clone();
        output.method = HttpMethod::Get;
        Ok(output)
    }
}
