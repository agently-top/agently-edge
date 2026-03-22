use anyhow::{Error, Result};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// 工具 trait
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, args: &JsonValue) -> Result<JsonValue>;
}

/// 工具注册表
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    /// 创建新注册表
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// 注册工具
    pub fn register(&mut self, tool: Box<dyn Tool>) -> Result<()> {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
        Ok(())
    }

    /// 检查工具是否存在
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// 执行工具
    pub fn execute(&self, name: &str, args: &JsonValue) -> Result<JsonValue> {
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| Error::msg(format!("Tool not found: {}", name)))?;
        tool.execute(args)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
