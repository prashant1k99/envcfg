use envcfg::EnvCfg;

#[derive(EnvCfg)]
pub struct DbConfig {
    host: String,
    port: String,
}

fn main() {}
