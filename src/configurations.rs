#![allow(unused)]
use serde::Deserialize;
use crate::engine::models::Dotfiles;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub app_config: AppConfig,
    pub dotfiles: Dotfiles,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub base_path: String
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Fail to determinate the current directory");
    let configuration_directory = base_path.join(".");

    let directive = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("configuration.yaml"),
        ))
        .build()?;

    directive.try_deserialize::<Configuration>()
}