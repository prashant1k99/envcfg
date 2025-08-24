use envcfg::EnvCfg;

#[derive(EnvCfg)]
pub struct DbConfig {
    host: String,
    port: String,
}

fn main() {
    let cfg = DbConfig::builder().load_from_env().build().unwrap();

    assert_eq!(cfg.host(),);
}
