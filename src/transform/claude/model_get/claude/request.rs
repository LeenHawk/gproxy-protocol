use crate::claude::model_get::request::ClaudeModelGetRequest;
use crate::claude::types::HttpMethod;
use crate::transform::utils::TransformError;

impl TryFrom<&ClaudeModelGetRequest> for ClaudeModelGetRequest {
    type Error = TransformError;

    fn try_from(value: &ClaudeModelGetRequest) -> Result<Self, TransformError> {
        let mut output = value.clone();
        output.method = HttpMethod::Get;
        Ok(output)
    }
}
