use std::{path::Path, sync::Arc};

use anyhow::Result;

use crate::{config::ServerConfig, filesystem::ProjectFilesystem};

#[derive(Clone)]
pub struct AppState {
    pub filesystem: Arc<ProjectFilesystem>,
}

impl AppState {
    pub fn new(config: &ServerConfig) -> Result<Self> {
        Ok(Self {
            filesystem: Arc::new(ProjectFilesystem::new(Path::new(&config.project_root))?),
        })
    }
}
