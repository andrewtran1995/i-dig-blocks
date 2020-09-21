use config::{ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Secrets {
    pub cubed_host: CubedHost,
    pub discord: Discord,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CubedHost {
    pub api_key: String,
    pub api_user: String,
    pub server_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Discord {
    pub client_id: u64,
    pub client_secret: String,
    pub bot_token: String,
}

impl Secrets {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = config::Config::default();
        s.merge(File::with_name("secrets.toml"))?;
        s.try_into()
    }
}
