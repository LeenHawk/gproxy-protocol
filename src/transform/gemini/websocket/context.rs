#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GeminiWebsocketTransformContext {
    pub warnings: Vec<String>,
}

impl GeminiWebsocketTransformContext {
    pub fn push_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }
}
