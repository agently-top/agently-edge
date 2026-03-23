//! 对话循环测试
//!
//! 测试 ConversationContext、退出命令检测和欢迎信息生成

use hello_world_agent::{build_welcome, load_config, should_exit, ConversationContext};

/// 测试 1: 多轮对话历史
#[test]
fn test_conversation_history() {
    let mut context = ConversationContext::new(20); // max 20 turns

    context.add_user("Hello".to_string());
    context.add_assistant("Hi there!".to_string());
    context.add_user("How are you?".to_string());

    let history = context.get_history();
    assert_eq!(history.len(), 3); // 3 messages in history
}

/// 测试 2: 退出命令检测
#[test]
fn test_exit_command() {
    assert!(should_exit("quit"));
    assert!(should_exit("exit"));
    assert!(should_exit("QUIT")); // case insensitive
    assert!(!should_exit("hello"));
}

/// 测试 3: 欢迎信息生成
#[test]
fn test_welcome_message() {
    let config = load_config("agent.yaml").unwrap();
    let welcome = build_welcome(&config.prompts.welcome);
    assert!(welcome.contains("Hello"));
    assert!(welcome.contains("agently Edge Runtime"));
}
