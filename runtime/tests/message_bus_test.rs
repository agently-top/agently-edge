use agently_edge::message_bus::{Message, MessageBus};
use std::time::Duration;

#[test]
fn test_message_bus_subscribe_publish() {
    let bus = MessageBus::new();

    // 订阅主题
    let rx = bus.subscribe("test-topic");

    // 发布消息
    let msg = Message {
        topic: "test-topic".to_string(),
        payload: "Hello, World!".to_string(),
    };
    bus.publish("test-topic", msg).expect("Failed to publish");

    // 接收消息
    let received = rx
        .recv_timeout(Duration::from_millis(100))
        .expect("Failed to receive message");
    assert_eq!(received.payload, "Hello, World!");
}

#[test]
fn test_message_bus_multiple_subscribers() {
    let bus = MessageBus::new();

    // 多个订阅者
    let rx1 = bus.subscribe("multi-topic");
    let rx2 = bus.subscribe("multi-topic");

    // 发布消息
    let msg = Message {
        topic: "multi-topic".to_string(),
        payload: "Broadcast".to_string(),
    };
    bus.publish("multi-topic", msg).expect("Failed to publish");

    // 两个订阅者都应该收到消息
    let msg1 = rx1.recv_timeout(Duration::from_millis(100)).unwrap();
    let msg2 = rx2.recv_timeout(Duration::from_millis(100)).unwrap();
    assert_eq!(msg1.payload, "Broadcast");
    assert_eq!(msg2.payload, "Broadcast");
}

#[test]
fn test_message_bus_request_reply() {
    let bus = MessageBus::new();

    // 请求/响应
    let request = Message {
        topic: "request".to_string(),
        payload: "What's the time?".to_string(),
    };

    // MVP 阶段：简单返回固定响应
    let response = bus
        .request_reply("request", request, Duration::from_millis(100))
        .expect("Failed to get response");

    assert!(!response.payload.is_empty());
}
