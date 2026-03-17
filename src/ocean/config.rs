use serde_derive::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new());

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub frontend: Frontend,
    pub postgres: Postgres,
    pub telegram_bot: TelegramBot,
    pub watchdog: Watchdog,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub anonym_allowed: bool,
}

#[derive(Debug, Deserialize)]
pub struct Frontend {
    pub domen: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct TelegramBot {
    pub token: String,
    pub url: String,
    pub channel: String,
    pub admin_chat_id: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Watchdog {
    pub enabled: bool,
    pub anonym_token: String,
}

impl Config {
    pub fn new() -> Self {
        let config_file = "ocean/ocean.toml";

        let mut home_config_path = dirs::config_dir().unwrap();
        home_config_path.push(config_file);

        let config_path: String;

        if !home_config_path.exists() {
            let mut etc_config_path = PathBuf::new();
            etc_config_path.push("/etc");
            etc_config_path.push(config_file);

            if !etc_config_path.exists() {
                panic!(
                    "config path not exists: {}, {}",
                    home_config_path.to_str().unwrap(),
                    etc_config_path.to_str().unwrap()
                );
            } else {
                config_path = etc_config_path.to_str().unwrap().to_string();
            }
        } else {
            config_path = home_config_path.to_str().unwrap().to_string();
        }

        let config_text = fs::read_to_string(config_path).unwrap();
        toml::from_str(&config_text).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
