use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::path::PathBuf;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug)]
pub enum Environment {
    Development,
    Test,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl TryFrom<String> for Environment {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "production" => Ok(Environment::Production),
            "development" => Ok(Environment::Development),
            "test" => Ok(Environment::Test),
            _ => Err("could not parse environment variable.(production, development, test)"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub port: u16,
}

impl ApplicationSettings {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub db_name: String,
    pub ssl_mode: bool
}

pub fn get_configuration(env: Option<String>, path: PathBuf) -> Result<Settings, ConfigError> {
    // If no environment passed, pass default
    let env = env.map_or(Default::default(), |e| {
        e.try_into().expect("could not parse environment")
    });
    let file_name = match env {
        Environment::Development => "config.dev.yml",
        Environment::Test => "config.test.yml",
        Environment::Production => "config.prod.yml",
    };
    let path = path.join(file_name);
    let path_str = path.to_str().expect("could not get path to config");
    // Building config
    let config = Config::builder()
        .add_source(File::new(path_str, FileFormat::Yaml))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    config.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.ssl_mode {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
        .host(&self.host)
        .port(self.port)
        .username(&self.username)
        .password(&self.password)
        .database(&self.db_name)
        .ssl_mode(ssl_mode)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.ssl_mode {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(ssl_mode)
    }
}
