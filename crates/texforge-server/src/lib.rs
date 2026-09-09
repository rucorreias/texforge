pub mod config;
mod filesystem;
pub mod routes;
pub mod state;

use config::ServerConfig;

pub async fn run(config: ServerConfig) -> anyhow::Result<()> {
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