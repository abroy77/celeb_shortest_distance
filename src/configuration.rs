use std::path::PathBuf;
use config;
use serde;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use sqlx::ConnectOptions;


#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub movie_data: MovieDataSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
   pub path: String 
}

#[derive(Deserialize, Clone)]
pub struct MovieDataSettings {
    pub file_path: PathBuf,
}

pub enum Environment {
    Local,
    Production,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current directory.");

    let config_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        // read the config file
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    //convert the config file into the Settings struct
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {

    pub fn connection_options(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new()
            .filename(&self.path)
    
    }
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "{} is not a valid environment. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}
