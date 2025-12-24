use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKeys {
    pub alpha_vantage: String,
    pub yahoo_finance: String,
    pub finnhub: String,
    pub news_api: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub api_keys: ApiKeys,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::builder();
        
        // Start with default values
        settings = settings
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default("database.url", "postgres://localhost/stock_analyst")?
            .set_default("database.max_connections", 10)?
            .set_default("api_keys.alpha_vantage", "demo")?
            .set_default("api_keys.yahoo_finance", "demo")?
            .set_default("api_keys.finnhub", "demo")?
            .set_default("api_keys.news_api", "demo")?;

        // Add in the current environment file
        settings = settings.add_source(File::with_name("config/default").required(false));
        
        // Add in a local configuration file
        settings = settings.add_source(File::with_name("config/local").required(false));

        // Add in settings from the environment (with a prefix of APP)
        settings = settings.add_source(config::Environment::with_prefix("APP"));

        let settings = settings.build()?;
        
        settings.try_deserialize()
    }
}