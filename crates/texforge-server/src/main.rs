use texforge_server::{config::ServerConfig, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = ServerConfig::from_env()?;
    run(config).await
}
