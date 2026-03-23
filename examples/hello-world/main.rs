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

use agently_edge::llm::{LLMEngine, Message, PromptManager};
use serde::Deserialize;
use std::fs;

/// Agent 配置结构
#[derive(Debug, Deserialize)]
struct AgentConfig {
    agent: AgentInfo,
    runtime: RuntimeConfig,
    prompts: Prompts,
}

#[derive(Debug, Deserialize)]
struct AgentInfo {
    id: String,
    name: String,
    version: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct RuntimeConfig {
    model_path: String,
    context_length: u32,
    temperature: f32,
    max_tokens: u32,
    n_threads: u32,
}

#[derive(Debug, Deserialize)]
struct Prompts {
    system: String,
    welcome: String,
}

/// 加载配置文件
fn load_config(path: &str) -> anyhow::Result<AgentConfig> {
    let content = fs::read_to_string(path)?;
    let config: AgentConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// 打印欢迎信息
fn print_welcome(config: &AgentConfig) {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  {} v{}                            ║", 
        format!("{:<48}", config.agent.name), 
        config.agent.version
    );
    println!("║  {:<48} ║", config.agent.description);
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", config.prompts.welcome);
    println!();
    println!("Type 'quit' or 'exit' to stop.\n");
}

/// 主函数
fn main() -> anyhow::Result<()> {
    // 1. 加载配置
    println!("📖 Loading configuration...");
    let config = load_config("agent.yaml")?;
    println!("✅ Configuration loaded: {}", config.agent.name);
    println!();

    // 2. 创建 LLM 引擎（使用 mock 模式，实际部署时替换为真实引擎）
    println!("🤖 Initializing LLM engine...");
    let mut engine = LLMEngine::mock();
    println!("✅ LLM engine initialized (mock mode)");
    println!();

    // 3. 创建 Prompt 管理器
    let mut prompt_mgr = PromptManager::new();
    prompt_mgr.add_system(&config.prompts.system);

    // 4. 打印欢迎信息
    print_welcome(&config);

    // 5. 对话循环
    loop {
        print!("👤 You: ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // 检查退出命令
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!();
            println!("👋 Goodbye! Thanks for using {}.", config.agent.name);
            break;
        }

        // 跳过空输入
        if input.is_empty() {
            continue;
        }

        // 添加用户消息
        prompt_mgr.add_user(input);

        // 构建消息
        let messages = prompt_mgr.build_messages();

        // 调用 LLM
        print!("🤖 Agent: ");
        io::stdout().flush()?;
        
        match engine.chat(&messages) {
            Ok(response) => {
                println!("{}", response);
                
                // 添加助手回复到历史
                prompt_mgr.add_assistant(&response);
            }
            Err(e) => {
                println!("(Error: {})", e);
            }
        }
        println!();
    }

    Ok(())
}
