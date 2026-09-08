mod config;
mod filesystem;
mod routes;
mod state;

use config::ServerConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = ServerConfig::from_env()?;
    let app = routes::router(state::AppState::new(&config)?);

    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;

    tracing::info!(
        address = %config.bind_addr,
        project_root = %config.project_root.display(),
        "TexForge server is running"
    );

    axum::serve(listener, app).await?;

    Ok(())
}
