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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::ServerConfig, state::AppState};
    use axum::{body::{to_bytes, Body}, http::Request};
    use serde_json::Value;
    use std::fs;
    use tempfile::TempDir;
    use tower::ServiceExt;

    fn test_app() -> (Router, TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        fs::create_dir(temp_dir.path().join("chapters")).unwrap();
        fs::write(temp_dir.path().join("main.tex"), "original").unwrap();
        fs::write(temp_dir.path().join("chapters/intro.tex"), "intro").unwrap();

        let config = ServerConfig {
            bind_addr: "127.0.0.1:0".parse().unwrap(),
            project_root: temp_dir.path().to_path_buf(),
        };
        let app = router(AppState::new(&config).unwrap());
        (app, temp_dir)
    }

    async fn response_json(response: Response) -> Value {
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let (app, _temp_dir) = test_app();
        let response = app
            .oneshot(Request::get("/api/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response_json(response).await, serde_json::json!({"status": "ok"}));
    }

    #[tokio::test]
    async fn lists_project_files() {
        let (app, _temp_dir) = test_app();
        let response = app
            .oneshot(Request::get("/api/files").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = response_json(response).await;

        assert_eq!(body["entries"][0]["path"], "chapters");
        assert!(body["entries"].as_array().unwrap().iter().any(|entry| entry["path"] == "main.tex"));
        assert!(body["entries"].as_array().unwrap().iter().any(|entry| entry["path"] == "chapters/intro.tex"));
    }

    #[tokio::test]
    async fn reads_existing_file() {
        let (app, _temp_dir) = test_app();
        let response = app
            .oneshot(Request::get("/api/files/main.tex").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = response_json(response).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, serde_json::json!({"path": "main.tex", "content": "original"}));
    }

    #[tokio::test]
    async fn rejects_missing_and_invalid_read_paths() {
        for path in ["missing.tex", "%2E%2E/outside.tex", "%2Ftmp%2Foutside.tex"] {
            let (app, _temp_dir) = test_app();
            let response = app
                .oneshot(Request::get(format!("/api/files/{path}")).body(Body::empty()).unwrap())
                .await
                .unwrap();

            let expected_status = if path == "missing.tex" { StatusCode::NOT_FOUND } else { StatusCode::BAD_REQUEST };
            assert_eq!(response.status(), expected_status, "unexpected status for {path}");
        }
    }

    #[tokio::test]
    async fn writes_file_and_persists_content() {
        let (app, temp_dir) = test_app();
        let response = app
            .oneshot(
                Request::put("/api/files/main.tex")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"content":"updated"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(fs::read_to_string(temp_dir.path().join("main.tex")).unwrap(), "updated");
    }

    #[tokio::test]
    async fn rejects_invalid_write_paths() {
        for path in ["%2E%2E/outside.tex", "%2Ftmp%2Foutside.tex"] {
            let (app, _temp_dir) = test_app();
            let response = app
                .oneshot(
                    Request::put(format!("/api/files/{path}"))
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"content":"updated"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST, "unexpected status for {path}");
        }
    }
}