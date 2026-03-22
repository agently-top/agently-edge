//! agently Edge Runtime
//!
//! Edge AI Agent 运行时，支持 Linux、Android、鸿蒙 Next 三平台。

pub mod agent;
pub mod config;
pub mod ffi;
pub mod llm;
pub mod message_bus;
pub mod runtime;
pub mod tools;

/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 初始化运行时
pub fn init() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    Ok(())
}
