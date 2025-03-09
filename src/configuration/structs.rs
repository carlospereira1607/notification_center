use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfiguration {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings{
    pub incoming: IncomingSettings
}

#[derive(Debug, Deserialize)]
pub struct IncomingSettings {
    pub rest: RestSettings
}

#[derive(Debug, Deserialize)]
pub struct RestSettings {
    pub port: u16
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u16,
}