use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{filesystem::{EntryType, FilesystemError}, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/files", get(list_files))
        .route("/api/files/{*path}", get(read_file).put(write_file))
        .with_state(state)
}

#[derive(Serialize)]
struct HealthResponse { status: &'static str }

async fn health() -> Json<HealthResponse> { Json(HealthResponse { status: "ok" }) }

#[derive(Serialize)]
struct FileListResponse { entries: Vec<FileEntryResponse> }

#[derive(Serialize)]
struct FileEntryResponse {
    name: String,
    path: String,
    #[serde(rename = "type")]
    entry_type: &'static str,
}

async fn list_files(State(state): State<AppState>) -> Result<Json<FileListResponse>, ApiError> {
    let entries = state.filesystem.list().map_err(ApiError::from_filesystem)?.into_iter().map(|entry| FileEntryResponse {
        name: entry.name,
        path: entry.path,
        entry_type: match entry.entry_type { EntryType::File => "file", EntryType::Directory => "directory" },
    }).collect();
    Ok(Json(FileListResponse { entries }))
}

#[derive(Serialize)]
struct FileResponse { path: String, content: String }

async fn read_file(State(state): State<AppState>, Path(path): Path<String>) -> Result<Json<FileResponse>, ApiError> {
    let content = state.filesystem.read_file(&path).map_err(ApiError::from_filesystem)?;
    Ok(Json(FileResponse { path, content }))
}

#[derive(Deserialize)]
struct WriteFileRequest { content: String }

async fn write_file(State(state): State<AppState>, Path(path): Path<String>, Json(request): Json<WriteFileRequest>) -> Result<Json<FileResponse>, ApiError> {
    state.filesystem.write_file(&path, &request.content).map_err(ApiError::from_filesystem)?;
    Ok(Json(FileResponse { path, content: request.content }))
}

struct ApiError(StatusCode, &'static str);

impl ApiError {
    fn from_filesystem(error: FilesystemError) -> Self {
        match error {
            FilesystemError::InvalidPath | FilesystemError::OutsideProject => Self(StatusCode::BAD_REQUEST, "invalid project path"),
            FilesystemError::Io(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Self(StatusCode::NOT_FOUND, "file not found"),
                std::io::ErrorKind::PermissionDenied => Self(StatusCode::FORBIDDEN, "permission denied"),
                _ => Self(StatusCode::INTERNAL_SERVER_ERROR, "filesystem error"),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response { (self.0, Json(serde_json::json!({ "error": self.1 }))).into_response() }
}