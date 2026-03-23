//! agently CLI - Edge AI Agent Runtime

use clap::{Parser, Subcommand};
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
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            config_path,
            verbose,
        } => {
            run_agent(config_path, verbose)?;
        }
    }

    Ok(())
}

fn run_agent(config_path: PathBuf, verbose: bool) -> anyhow::Result<()> {
    // 1. 加载配置
    let config = agently_edge::AgentConfig::load(&config_path)?;

    // 2. 打印信息（verbose 模式）
    if verbose {
        println!("Loading agent: {}", config.agent.name);
        println!("Config: {:?}", config_path);
    }

    // 3. 打印欢迎信息
    println!("{}", config.prompts.welcome);

    // TODO: 未来这里调用 runtime 的 Agent 运行接口
    // 目前仅作为 MVP 演示

    Ok(())
}
