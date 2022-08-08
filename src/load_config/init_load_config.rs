use schemars::schema::RootSchema;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::load_config::models::{EnvConfig, GlobalConfig};
use tracing::info;

fn load_env_config() -> Option<EnvConfig> {
    load_config::<EnvConfig>("application.yml")
}

fn load_global_config_from_env(active: String) -> Option<GlobalConfig> {
    let path = format!("application-{}.yml", active);
    load_config::<GlobalConfig>(&path)
}

pub fn load_global_config() -> Option<GlobalConfig> {
    if let Some(env_config) = load_env_config() {
        return load_global_config_from_env(env_config.profiles.active);
    }
    None
}

fn load_config<T>(path: &str) -> Option<T> where T: DeserializeOwned {
    match serde_yaml::from_str::<RootSchema>(&std::fs::read_to_string(path).expect(&format!("failure read file {}", path))) {
        Ok(root_schema) => {
            let data = serde_json::to_string_pretty(&root_schema).expect("");
            let config = serde_json::from_str::<T>(&*data).expect("");
            Some(config)
        }
        Err(err) => {
            info!("{}",err);
            None
        }
    }
}