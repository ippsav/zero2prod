use std::path::PathBuf;

use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

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
    pub port: u16,
}

impl ApplicationSettings {
    pub fn get_address(&self) -> String {
        format!("{}:{}",self.host,self.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
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
        .build()?;

    config.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }

    pub fn get_connection_without_database_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}