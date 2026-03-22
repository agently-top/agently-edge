use super::Tool;
use anyhow::Result;
use serde_json::Value as JsonValue;

/// HTTP 工具
pub struct HttpTool;

impl HttpTool {
    pub fn new() -> Self {
        Self
    }
}

impl Tool for HttpTool {
    fn name(&self) -> &str {
        "http"
    }

    fn description(&self) -> &str {
        "Make HTTP requests"
    }

    fn execute(&self, _args: &JsonValue) -> Result<JsonValue> {
        // MVP 阶段：返回固定响应
        Ok(JsonValue::String("HTTP response (MVP stub).".to_string()))
    }
}

impl Default for HttpTool {
    fn default() -> Self {
        Self::new()
    }
}
