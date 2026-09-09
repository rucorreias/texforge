use std::{env, net::SocketAddr, path::PathBuf};

use anyhow::Context;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    pub frontend_url: String,
    pub project_root: PathBuf,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
            frontend_url: String::from("http://127.0.0.1:5173"),
            project_root: PathBuf::from("."),
        }
    }
}

impl ServerConfig {
    pub fn url(&self) -> String {
        format!("http://{}", self.bind_addr)
    }

    pub fn from_env() -> anyhow::Result<Self> {
        let project_root = env::var_os("TEXFORGE_PROJECT_ROOT")
            .map(PathBuf::from)
            .unwrap_or(env::current_dir().context("failed to determine current directory")?);
        Self::from_project_root(project_root)
    }

    pub fn from_project_root(project_root: PathBuf) -> anyhow::Result<Self> {
        let project_root = std::fs::canonicalize(&project_root)
            .with_context(|| format!("failed to resolve project root: {}", project_root.display()))?;
        if !project_root.is_dir() {
            anyhow::bail!("project root is not a directory: {}", project_root.display());
        }

        let mut config = Self {
            project_root,
            ..Self::default()
        };
        if let Some(frontend_url) = env::var_os("TEXFORGE_FRONTEND_URL") {
            config.frontend_url = frontend_url.to_string_lossy().into_owned();
        }
        Ok(config)
    }
}
