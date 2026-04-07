extern crate diesel_migrations;
use diesel_migrations::MigrationHarness;
use ocean::api::user_cache;
use ocean::app;
use ocean::db;
use tracing_subscriber::{EnvFilter, prelude::*};

use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn init_tracing() {
    let filter_layer = EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer().json();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let mut db = db::Db::new();
    db.conn.run_pending_migrations(MIGRATIONS)?;

    user_cache::init(db);

    let app = app::App::new();
    app.start().await?;
    Ok(())
}
