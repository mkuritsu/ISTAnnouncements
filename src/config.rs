use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub username: String,
    pub avatar_url: String,
    pub webhook_url: String,
    pub mention_role: u64,
    pub poll_time: u64,
    pub database_url: String,
}

impl AppConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<AppConfig> {
        let toml_str = fs::read_to_string(path)?;
        let app_config = toml::from_str(&toml_str)?;
        Ok(app_config)
    }

    pub async fn save_to_file_async<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let toml_str = toml::to_string(self)?;
        tokio::fs::write(path, toml_str).await?;
        Ok(())
    }
}
