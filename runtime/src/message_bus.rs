//! 消息总线模块

use anyhow::Result;

/// 消息
pub struct Message {
    pub topic: String,
    pub payload: String,
}

/// 消息总线
pub struct MessageBus;

impl MessageBus {
    /// 创建新消息总线
    pub fn new() -> Self {
        Self
    }
    
    /// 发布消息
    pub fn publish(&self, message: Message) -> Result<()> {
        tracing::debug!("Publish message to topic: {}", message.topic);
        Ok(())
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
