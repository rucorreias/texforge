use clap::Parser;
use std::path::PathBuf;
use texforge_server::{config::ServerConfig, run};

#[derive(Parser)]
#[command(name = "texforge")]
#[command(about = "A local-first LaTeX IDE")]
struct Cli {
    #[arg(default_value = ".")]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = ServerConfig::from_project_root(cli.path)?;

    println!("TexForge project: {}", config.project_root.display());
    println!("Server: http://{}", config.bind_addr);

    run(config).await
}