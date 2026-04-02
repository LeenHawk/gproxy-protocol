#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpenAiWebsocketTransformContext {
    pub warnings: Vec<String>,
}

impl OpenAiWebsocketTransformContext {
    pub fn push_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }
}
