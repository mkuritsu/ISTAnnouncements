use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub username: String,
    pub avatar_url: String,
    pub webhook_url: String,
    pub poll_time: u64,
    pub database_url: String,
    pub web_dir: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<AppConfig> {
        let toml_str = fs::read_to_string(path)?;
        toml::from_str(&toml_str).map_err(|e| e.into())
    }

    pub async fn save_to_file_async<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let toml_str = toml::to_string(self)?;
        tokio::fs::write(path, toml_str).await.map_err(|e| e.into())
    }
}
