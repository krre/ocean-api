use crate::config;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct Db {
    pub conn: PgConnection,
}

impl Db {
    pub fn new() -> Db {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config::CONFIG.postgres.username,
            config::CONFIG.postgres.password,
            config::CONFIG.postgres.host,
            config::CONFIG.postgres.port,
            config::CONFIG.postgres.database
        );
        let conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Db { conn }
    }
}

impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}
