pub mod config;
mod filesystem;
pub mod routes;
pub mod state;

use config::ServerConfig;

pub async fn start(config: ServerConfig) -> anyhow::Result<tokio::task::JoinHandle<anyhow::Result<()>>> {
    let app = routes::router(state::AppState::new(&config)?);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;

    tracing::info!(
        address = %config.url(),
        project_root = %config.project_root.display(),
        "TexForge server is running"
    );

    Ok(tokio::spawn(async move {
        axum::serve(listener, app).await.map_err(anyhow::Error::from)
    }))
}

pub async fn run(config: ServerConfig) -> anyhow::Result<()> {
    start(config).await?.await??;
    Ok(())
}