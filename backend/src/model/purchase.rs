use crate::model;
use crate::model::db::Db;
use serde_json::json;
use serde_json::Value as JsonValue;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Purchase {
    pub id: i64,
    pub items: JsonValue,
    pub total: i64,
}

#[derive(Default, Clone)]
pub struct PurchasePatch {
    pub items: Option<JsonValue>,
    pub total: Option<i64>,
}

// Mac: model access controller
pub struct PurchaseMac;

impl PurchaseMac {
    pub async fn create(db: &Db, data: PurchasePatch) -> Result<Purchase, model::Error> {
        let sql = "INSERT INTO purchase (items, total) VALUES ($1, $2) RETURNING id, items, total";
        let query = sqlx::query_as(sql).bind(json!([])).bind(0);

        let purchase = query.fetch_one(db).await?;

        Ok(purchase)
    }

    pub async fn list(db: &Db) -> Result<Vec<Purchase>, model::Error> {
        let sql = "SELECT id, items, total FROM purchase ORDER BY id DESC";

        // build sqlx query
        let query = sqlx::query_as(sql);
        // execute query
        let purchases = query.fetch_all(db).await?;

        Ok(purchases)
    }
}

#[cfg(test)]
#[path = "../_tests/model_purchase.rs"]
mod tests;
