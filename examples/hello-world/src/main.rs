//! Hello World Agent - 示例应用
//!
//! 这是一个简单的对话 Agent 示例，展示如何使用 agently Edge Runtime。
//!
//! 功能：
//! - 加载配置文件
//! - 初始化运行时
//! - 交互式对话
//!
//! 使用方法：
//! ```bash
//! cargo run
//! ```

use hello_world_agent::{build_welcome, load_config, should_exit, ConversationContext};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    // 1. 加载配置
    let config = load_config("agent.yaml")?;

    // 2. 打印欢迎信息
    println!("{}", build_welcome(&config.prompts.welcome));

    // 3. 创建对话上下文
    let mut context = ConversationContext::new(20);

    // 4. 对话循环
    let mut input = String::new();
    loop {
        print!("👤 You: ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // 检查退出命令
        if should_exit(input) {
            println!("👋 Goodbye! Thanks for using Hello World Agent.");
            break;
        }

        // 添加到上下文
        context.add_user(input.to_string());

        // Mock 响应（未来替换为真实 LLM 调用）
        let response = format!("Mock response to: {}", input);
        context.add_assistant(response.clone());

        println!("🤖 Agent: {}", response);
    }

    Ok(())
}
