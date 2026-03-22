//! Shell 工具

use anyhow::Result;

/// 执行 Shell 命令
pub fn execute(cmd: &str) -> Result<String> {
    tracing::debug!("Shell command: {}", cmd);
    Ok("".to_string())
}
