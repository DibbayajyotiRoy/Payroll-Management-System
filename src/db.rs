use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn init_db() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Neon DB")
}
