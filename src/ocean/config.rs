use serde_derive::Deserialize;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new());

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub postgres_url: String,

    pub anonym_allowed: bool,
    pub frontend_domen: String,

    pub telegram_bot_url: String,
    pub telegram_bot_token: String,
    pub telegram_bot_channel: String,
    pub telegram_bot_admin_chat_id: String,
    pub telegram_bot_enabled: bool,

    pub watchdog_anonym_token: String,
    pub watchdog_enabled: bool,
}

impl Config {
    fn new() -> Self {
        envy::from_env().expect("Failed to load config")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
