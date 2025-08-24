use std::collections::hash_map;

use envcfg::EnvCfg;

#[derive(EnvCfg)]
pub struct DbConfig {
    db_username: String,
    db_password: String,
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

fn main() {
    let db_cfg = DbConfig {
        db_username: String::from("MyUsername"),
        db_password: String::from("MySecretPassword"),
    };

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
