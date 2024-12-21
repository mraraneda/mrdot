#![allow(unused)]

use crate::engine::models::Dotfiles;
use config::Config;
use regex::Regex;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub app_config: AppConfig,
    pub dotfiles: Dotfiles,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Fail to determinate the current directory");
    let configuration_directory = base_path.join(".");

    let conf = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("configuration.yaml"),
        ))
        .build()?;

    conf.try_deserialize::<Configuration>()
}
// 
// fn expand_env_vars(path: &String) -> String {
//     // Regex para encontrar variables de entorno en la forma $VARIABLE
//     let re = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)")?;
// 
//     // Sustituir cada variable de entorno por su valor correspondiente
//     let expanded_path = re.replace_all(&path, |caps: &regex::Captures| {
//         // Obtener el nombre de la variable (sin el $)
//         let var_name = &caps[1];
//         // Buscar el valor de la variable de entorno
//         env::var(var_name).unwrap_or_else(|_| format!("${}", var_name))
//     });
// 
//     expanded_path.to_string()
// }
//