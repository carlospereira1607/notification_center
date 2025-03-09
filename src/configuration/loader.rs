use crate::configuration::structs::AppConfiguration;
use config::{Config, ConfigError, Environment, File};

pub fn load_config() -> Result<AppConfiguration, ConfigError> {
    let config_builder = Config::builder()
        .add_source(File::with_name("configuration.yaml"))
        .add_source(Environment::default().separator("_"));

    Ok(config_builder.build()?.try_deserialize()?)
}