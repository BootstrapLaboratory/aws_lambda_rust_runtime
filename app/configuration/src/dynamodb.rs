use std::env;

// Application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Optional custom endpoint for DynamoDB.
    pub endpoint: Option<String>,
}

// Constructs the configuration from environment variables.
pub fn load_config() -> Config {
    let endpoint = env::var("DYNAMODB_ENDPOINT").ok();
    Config { endpoint }
}

use config_macro::define_global_config;
define_global_config!(Config, load_config);
