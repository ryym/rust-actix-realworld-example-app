#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret_key: String,
    pub frontend_origin: Option<String>,
}

pub trait HaveConfig {
    fn config(&self) -> &Config;
}
