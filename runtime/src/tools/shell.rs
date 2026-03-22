use super::Tool;
use anyhow::Result;
use serde_json::Value as JsonValue;

/// Shell 工具
pub struct ShellTool;

impl ShellTool {
    pub fn new() -> Self {
        Self
    }
}

impl Tool for ShellTool {
    fn name(&self) -> &str {
        "shell"
    }

    fn description(&self) -> &str {
        "Execute shell commands"
    }

    fn execute(&self, args: &JsonValue) -> Result<JsonValue> {
        // MVP 阶段：简单返回命令
        if let Some(cmd) = args.get("command").and_then(|v| v.as_str()) {
            Ok(JsonValue::String(format!("Executed: {}", cmd)))
        } else {
            Ok(JsonValue::String("No command provided".to_string()))
        }
    }
}

impl Default for ShellTool {
    fn default() -> Self {
        Self::new()
    }
}
