mod config;
mod state;

use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use config::ServerConfig;
use serde::Serialize;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = ServerConfig::default();
    let state = AppState::new();
    let app = Router::new()
        .route("/api/health", get(health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;

    tracing::info!(address = %config.bind_addr, "TexForge server is running");

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health(State(_state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
