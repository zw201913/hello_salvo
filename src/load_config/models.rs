use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub mysql: Mysql,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub active: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvConfig {
    pub profiles: Profiles,
}