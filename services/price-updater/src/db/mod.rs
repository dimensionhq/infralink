pub mod insert;

use anyhow::Result;
use sqlx::PgPool;

pub async fn connect() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Ok(PgPool::connect(&database_url).await?)
}
