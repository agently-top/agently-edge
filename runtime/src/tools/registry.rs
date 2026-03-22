//! 工具注册表

use anyhow::Result;

/// 工具注册表
pub struct ToolRegistry {}

impl ToolRegistry {
    /// 创建新注册表
    pub fn new() -> Self {
        Self {}
    }
    
    /// 注册工具
    pub fn register(&mut self, _name: &str, _tool: Box<dyn Fn() -> Result<()>>) -> Result<()> {
        Ok(())
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
