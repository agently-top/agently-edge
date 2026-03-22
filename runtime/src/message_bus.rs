//! 消息总线模块

use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;

/// 消息
#[derive(Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: String,
}

/// 订阅者接收器（包装 tokio broadcast::Receiver 提供同步 API）
pub struct Receiver {
    inner: RefCell<broadcast::Receiver<Message>>,
}

impl Receiver {
    fn new(inner: broadcast::Receiver<Message>) -> Self {
        Self {
            inner: RefCell::new(inner),
        }
    }

    /// 带超时的接收（同步阻塞）
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Message, broadcast::error::RecvError> {
        let mut rx = self.inner.borrow_mut();
        let start = std::time::Instant::now();
        loop {
            match rx.try_recv() {
                Ok(msg) => return Ok(msg),
                Err(broadcast::error::TryRecvError::Empty) => {
                    if start.elapsed() > timeout {
                        return Err(broadcast::error::RecvError::Lagged(0));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                Err(broadcast::error::TryRecvError::Closed) => {
                    return Err(broadcast::error::RecvError::Closed);
                }
                Err(broadcast::error::TryRecvError::Lagged(n)) => {
                    return Err(broadcast::error::RecvError::Lagged(n));
                }
            }
        }
    }
}

/// 消息总线
pub struct MessageBus {
    channels: Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>,
}

impl MessageBus {
    /// 创建新消息总线
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 订阅主题
    pub fn subscribe(&self, topic: &str) -> Receiver {
        let mut channels = self.channels.lock().unwrap();

        let tx = channels
            .entry(topic.to_string())
            .or_insert_with(|| broadcast::channel(100).0);

        Receiver::new(tx.subscribe())
    }

    /// 发布消息
    pub fn publish(&self, topic: &str, message: Message) -> Result<()> {
        let channels = self.channels.lock().unwrap();

        if let Some(tx) = channels.get(topic) {
            let _ = tx.send(message);
        }

        Ok(())
    }

    /// 请求/响应（MVP 阶段：简单返回固定响应）
    pub fn request_reply(
        &self,
        topic: &str,
        _request: Message,
        _timeout: Duration,
    ) -> Result<Message> {
        // MVP 阶段：简单返回固定响应
        let response = Message {
            topic: topic.to_string(),
            payload: "Response from message bus (MVP).".to_string(),
        };
        Ok(response)
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
