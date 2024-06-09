#![allow(unused)]
use serde::Deserialize;
use crate::engine::models::FsKind;

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub base_path: String,
    pub in_case_collision: CollisionOptions,
}


#[derive(Deserialize, Debug)]
pub enum CollisionOptions {
    Replace,
    Omit,
}

// #[derive(serde::Deserialize, Debug)]
// pub struct Homebrew {
//     formulae: Vec<String>,
//     casks: Vec<String>,
// }

#[derive(Deserialize, Debug)]
pub struct Directive {
    targets: Vec<Target>
}

#[derive(Deserialize, Debug)]
pub struct Target {
    application: String,
    files: Vec<File>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    path: String,
    kind: String,
}


#[derive(Deserialize, Debug)]
pub struct Targets {

}


#[derive(Deserialize, Debug)]
pub struct Element {
    path: String,
    command: String,
}

pub fn get_configurations() -> Result<Configs, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Fail to determinate the current directory");
    let configuration_directory = base_path.join(".");

    let configurations = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("./configurations.yaml"),
        ))
        .build()?;

    configurations.try_deserialize::<Configs>()
}

pub fn get_directive() -> Result<Directive, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Fail to determinate the current directory");
    let configuration_directory = base_path.join(".");

    let directive = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("./configurations.yaml"),
        ))
        .build()?;

    directive.try_deserialize::<Directive>()
}