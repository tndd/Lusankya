use std::sync::{Arc, RwLock};
use sqlx::postgres::PgPool;
use sqlx::types::Json;
use crate::domain::asset::entity::Asset;
use crate::infra::adapter::asset::assets_to_json;

pub async fn insert_assets(pool: &PgPool, assets: Vec<Asset>) -> Result<(), sqlx::Error> {
    let json = Json(assets_to_json(assets).unwrap());
    sqlx::query("INSERT INTO assets SELECT * FROM json_populate_recordset(NULL::assets, $1)")
        .bind(json)
        .execute(pool)
        .await?;

    Ok(())
}