//! agently CLI - Edge AI Agent Runtime

use agently_edge::config::AgentConfig;
use agently_edge::llm::{LLMEngine, Message, ModelConfig, GenerateConfig, Role};
use clap::{Parser, Subcommand};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "agently")]
#[command(about = "Edge AI Agent Runtime CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run an agent with the given configuration
    Run {
        /// Path to agent configuration file (YAML)
        config_path: PathBuf,

        /// Enable verbose output
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
        
        /// Use mock mode (no model file required)
        #[arg(long, default_value_t = false)]
        mock: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rt = tokio::runtime::Runtime::new()?;
    
    match cli.command {
        Commands::Run {
            config_path,
            verbose,
            mock,
        } => {
            rt.block_on(run_agent(config_path, verbose, mock))?;
        }
    }

    Ok(())
}

async fn run_agent(config_path: PathBuf, verbose: bool, mock: bool) -> anyhow::Result<()> {
    // 1. 加载配置
    let config = AgentConfig::load(&config_path)?;

    // 2. 打印信息
    println!("🚀 Starting {} v{}...", config.agent.name, config.agent.version);
    if verbose {
        println!("   Config: {:?}", config_path);
        println!("   Model: {}", config.runtime.model_path);
    }

    // 3. 初始化 LLM 引擎
    let model_config = ModelConfig {
        model_path: if mock { 
            String::new() 
        } else { 
            config.runtime.model_path.clone() 
        },
        n_ctx: config.runtime.context_length,
        n_threads: config.runtime.n_threads,
        n_gpu_layers: 0,
    };

    let mut engine = if mock {
        if verbose {
            println!("   Mode: 🎭 Mock (no model file required)");
        }
        LLMEngine::mock()
    } else {
        if verbose {
            println!("   Mode: 🔧 Real model");
        }
        LLMEngine::new(model_config)?
    };

    // 4. 加载模型
    if verbose {
        println!("   Loading model...");
    }
    let model_info = engine.load_model().await?;
    if verbose {
        println!("   ✅ Model loaded: {} ({:.1} MB)", model_info.name, model_info.size_mb);
    }

    // 5. 打印欢迎信息
    println!();
    println!("{}", config.prompts.welcome);
    println!();

    // 6. 交互式对话
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut messages: Vec<Message> = Vec::new();

    // 添加系统提示
    messages.push(Message {
        role: Role::System,
        content: config.prompts.system.clone(),
    });

    let generate_config = GenerateConfig {
        max_tokens: config.runtime.max_tokens,
        temperature: config.runtime.temperature,
        top_p: 0.9,
        stop_sequences: vec![],
    };

    loop {
        print!("You: ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // 检查退出命令
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("\n👋 Goodbye!");
            break;
        }

        // 添加用户消息
        messages.push(Message {
            role: Role::User,
            content: input.to_string(),
        });

        // 生成响应
        if verbose {
            println!("   [Generating response...]");
        }

        let result = engine.chat(&messages, &generate_config).await?;
        
        println!("\n{}: {}\n", config.agent.name, result.text);

        // 添加助手消息到历史
        messages.push(Message {
            role: Role::Assistant,
            content: result.text.clone(),
        });

        // 打印统计
        if verbose {
            let stats = engine.get_stats();
            println!("   [Tokens: {}, Speed: {:.1} t/s]", 
                     result.tokens_generated, stats.tokens_per_second);
        }
    }

    Ok(())
}
