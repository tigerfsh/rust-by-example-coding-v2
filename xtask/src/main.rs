use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "cargo xtask")]
#[command(about = "Custom build tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run code generation
    Gen,
    /// Run all tests
    TestAll,
    /// Deploy the project
    Deploy {
        /// Deployment target
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen => {
            println!("Running code generation...");
            // 实际生成代码的逻辑
        }
        Commands::TestAll => {
            println!("Running all tests...");
            // 运行测试的逻辑
        }
        Commands::Deploy { target } => {
            println!("Deploying to {}...", target);
            // 部署逻辑
        }
    }

    Ok(())
}
