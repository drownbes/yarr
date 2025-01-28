use std::path::Path;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub app: App,
    pub db: Db,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct App {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Db {
    pub conn_str: String,
}

pub fn read_config(cfg_path: &Path) -> Result<AppConfig, config::ConfigError> {
    let cfg = config::Config::builder()
        .add_source(config::File::from(cfg_path))
        .build()?;

    cfg.try_deserialize()
}
