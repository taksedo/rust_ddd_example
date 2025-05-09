use std::env;

use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use dotenvy::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}
