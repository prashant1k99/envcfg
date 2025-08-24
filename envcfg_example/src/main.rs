use envcfg::EnvCfg;

#[derive(EnvCfg)]
pub struct DbConfig {
    #[env_cfg(default = "4317")]
    db_host: String,
}

#[derive(EnvCfg)]
pub struct Config {
    #[env_cfg(nested)]
    db_config: DbConfig,
    #[env_cfg(default = "8080", env_name = "PORT")]
    port: String,
}

fn main() {
    let db_cfg = DbConfig {
        db_host: "4312".to_owned(),
    };
    let cfg = Config::builder()
        .init_from_env()
        .set_db_config(db_cfg)
        .build()
        .unwrap();
}
