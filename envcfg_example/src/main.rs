use std::collections::{HashMap, hash_map};

use envcfg::EnvCfg;

#[derive(EnvCfg)]
pub struct DbConfig {
    db_username: String,
    db_password: String,
    #[env_cfg(from = "DB_HOST")]
    db_host: String,
}

pub struct DbConfigBuilder {
    db_username: Option<String>,
    db_password: Option<String>,
    db_host: Option<String>,
}

impl DbConfigBuilder {
    fn load_from_env(&mut self) -> Self {
        todo!()
    }

    fn load_from_hashmap<T: Iterator>(&mut self, hm: HashMap<String, T>) -> Self {
        todo!()
    }

    fn set_db_username(&mut self, db_username: String) -> Self {
        todo!()
    }

    fn set_db_password(&mut self, db_password: String) -> Self {
        todo!()
    }

    fn set_db_host(&mut self, d_host: String) -> Self {
        todo!()
    }

    fn build(&self) -> Result<DbConfig, String> {
        todo!()
    }
}

impl DbConfig {
    fn builder() -> DbConfigBuilder {
        todo!()
    }

    fn db_password(&self) -> String {
        self.db_password.clone()
    }

    fn db_host(&self) -> String {
        self.db_host.clone()
    }

    fn db_username(&self) -> String {
        self.db_username.clone()
    }
}

#[derive(EnvCfg)]
pub struct Config {
    #[env_cfg(nested)]
    db_config: DbConfig,

    // To Load from env
    #[env_cfg(default = "8080", from = "PORT")]
    port: String,

    // To load from hashmap
    config_1: String,
    config_2: String,
    #[env_cfg(from = "cfg_3")]
    config_3: String,
}

impl Config {
    fn builder() -> ConfigBuilder {
        todo!()
    }

    fn db_config(&self) -> &DbConfig {
        &self.db_config
    }

    fn port(&self) -> String {
        self.port.clone()
    }
}

pub struct ConfigBuilder {
    db_config: Option<DbConfig>,
    port: Option<String>,
    config_1: Option<String>,
    config_2: Option<String>,
    config_3: Option<String>,
}

impl ConfigBuilder {
    fn load_from_env(&mut self) -> Self {
        todo!()
    }

    fn load_from_hashmap<T: Copy>(&mut self, hm: HashMap<&str, T>) -> Self {
        todo!()
    }

    fn set_db_config(&mut self, db_config: DbConfig) -> Self {
        todo!()
    }

    fn build(&self) -> Result<Config, String> {
        todo!()
    }
}

fn main() {
    let db_cfg = DbConfig::builder()
        .load_from_env()
        .set_db_username(String::from("MyUsername"))
        .set_db_password(String::from("MySecretPassword"))
        .build()
        .unwrap();

    let mut h_map = hash_map::HashMap::new();
    h_map.insert("config_1", "Value_1");
    h_map.insert("config_2", "Value_2");
    h_map.insert("cfg_3", "Value_3");

    let cfg = Config::builder()
        .load_from_env() // Load port from env variables
        .load_from_hashmap(h_map) // Load config_1, config_2, config_3 from hashmap
        .set_db_config(db_cfg) // To set custom struct value [DbConfig]
        .build()
        .unwrap();

    assert_eq!(cfg.port(), String::from("4312"));
    assert_eq!(cfg.db_config().db_username(), String::from("MyUsername"));

    // Should not be able to mutate values after .build() is called
}
