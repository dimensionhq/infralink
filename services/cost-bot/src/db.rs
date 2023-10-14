use std::collections::HashMap;

use indexmap::IndexMap;

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPool::connect(database_url).await.unwrap()
}

use serde_json::Value;
use sqlx::{Error, PgPool, Row};

pub async fn fetch_repository_cost_limits(
    pool: &PgPool,
    repository_id: i64,
) -> Result<IndexMap<String, f64>, Error> {
    let row = sqlx::query(
        "SELECT cost_limit FROM cost_limits WHERE repository_id = $1 ORDER BY cost_limit DESC",
    )
    .bind(repository_id)
    .fetch_one(pool)
    .await?;

    let cost_limit: Value = row.get("cost_limit");

    let parsed: IndexMap<String, f64> = serde_json::from_value(cost_limit).unwrap();

    Ok(parsed)
}

pub async fn store_breakdown(
    pool: &PgPool,
    repository_id: i64,
    commit_ref: &str,
    breakdowns: IndexMap<String, IndexMap<String, f64>>,
) -> Result<(), Error> {
    // Convert the nested IndexMap to a nested HashMap
    let breakdowns_hash_map: HashMap<_, _> = breakdowns
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect::<HashMap<_, _>>()))
        .collect();

    // Serialize the HashMap to a JSON string
    let breakdowns_json = serde_json::to_string(&breakdowns_hash_map).unwrap();

    // Prepare and execute the SQL INSERT statement
    sqlx::query(
        "INSERT INTO cost_runs (repository_id, commit_ref, cost_breakdown) VALUES ($1, $2, $3)",
    )
    .bind(repository_id)
    .bind(commit_ref)
    .bind(breakdowns_json)
    .execute(pool)
    .await?;

    // Return success
    Ok(())
}

pub async fn remove_breakdown(
    pool: &PgPool,
    repository_id: i64,
    commit_ref: &str,
) -> Result<(), Error> {
    sqlx::query("DELETE FROM cost_runs WHERE repository_id = $1 AND commit_ref = $2")
        .bind(repository_id)
        .bind(commit_ref)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn fetch_previous_breakdown(
    pool: &PgPool,
    repository_id: i64,
    commit_ref: &str,
) -> Result<Option<IndexMap<String, IndexMap<String, f64>>>, Error> {
    let row = sqlx::query(
        "SELECT cost_breakdown FROM cost_runs WHERE repository_id = $1 AND commit_ref = $2 LIMIT 1",
    )
    .bind(repository_id)
    .bind(commit_ref)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        let cost_breakdown: Value = row.get("cost_breakdown");

        // Convert Value to your desired IndexMap type
        let parsed: IndexMap<String, IndexMap<String, f64>> =
            serde_json::from_value(cost_breakdown).unwrap();

        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}
