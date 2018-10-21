use config::{Config, HaveConfig};

#[derive(Debug)]
pub struct Hub {
    config: Config,
}

impl Hub {
    pub fn create(config: Config) -> Hub {
        Hub { config }
    }
}

impl HaveConfig for Hub {
    fn config(&self) -> &Config {
        &self.config
    }
}
