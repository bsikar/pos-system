use chrono::Local;
use chrono::NaiveDateTime;
use crate::model::{self, db::Db, item::Item};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub id: i64,
    pub items: JsonValue,
    pub ctime: NaiveDateTime,
    pub total: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PurchasePatch {
    pub items: JsonValue,
}

impl PurchasePatch {
    pub async fn to_items(&self) -> Result<Vec<Item>, model::Error> {
        let mut items = vec![];

        if self.items == json!(null) || self.items == json!({}) || self.items == json!([]) {
            return Err(model::Error::EmptyItems);
        }

        for item in self.items.as_array().unwrap() {
            let name = item["name"].as_str().unwrap().to_string();
            let price = item["price"].as_i64().unwrap();

            items.push(Item::new(name, price));
        }

        Ok(items)
    }

    pub async fn validate(&self, db: &Db) -> Result<(), model::Error> {
        let items = self.to_items().await?;

        for item in items {
            item.validate(db).await?;
        }

        Ok(())
    }
}

// Mac: model access controller
pub struct PurchaseMac;

#[async_trait]
impl model::Database<Purchase, PurchasePatch, i64> for PurchaseMac {
    async fn create(db: &Db, data: PurchasePatch) -> Result<Purchase, model::Error> {
        let sql = "INSERT INTO purchase (ctime, items, total) VALUES ($1, $2, $3) RETURNING id, ctime, items, total";
        // validate data
        data.validate(db).await?;

        let time = Local::now().naive_local();
        let items = data.items;
        let total = calculate_total(&items);

        let query = sqlx::query_as(sql)
            .bind(time)
            .bind(items)
            .bind(total);

        let purchase = query.fetch_one(db).await?;

        Ok(purchase)
    }

    async fn get(db: &Db, id: i64) -> Result<Purchase, model::Error> {
        let sql = "SELECT id, ctime, items, total FROM purchase WHERE id = $1";
        let query = sqlx::query_as(sql).bind(id);

        let result = query.fetch_one(db).await;
        Self::handle_fetch_one_result(result, "purchase", id)
    }

    async fn update(db: &Db, id: i64, data: PurchasePatch) -> Result<Purchase, model::Error> {
        let sql =
            "UPDATE purchase SET ctime = $1, items = $2, total = $3 WHERE id = $4 RETURNING id, ctime, items, total";
        let time = Local::now().naive_local();
        let items = data.items;
        let total = calculate_total(&items);
        let query = sqlx::query_as(sql).bind(time).bind(items).bind(total).bind(id);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "purchase", id)
    }

    async fn list(db: &Db) -> Result<Vec<Purchase>, model::Error> {
        let sql = "SELECT id, ctime, items, total FROM purchase ORDER BY id DESC";

        // build sqlx query
        let query = sqlx::query_as(sql);
        // execute query
        let purchases = query.fetch_all(db).await?;

        Ok(purchases)
    }

    async fn delete(db: &Db, id: i64) -> Result<Purchase, model::Error> {
        let sql = "DELETE FROM purchase WHERE id = $1 RETURNING id, ctime, items, total";
        let query = sqlx::query_as(sql).bind(id);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "purchase", id)
    }
}

pub fn calculate_total(items: &JsonValue) -> i64 {
    let mut total = 0;

    if let Some(items) = items.as_array() {
        for item in items {
            total += item["price"].as_i64().unwrap() * item["quantity"].as_i64().unwrap();
        }
    }

    total
}

#[cfg(test)]
#[path = "../../tests/model_tests/purchase.rs"]
mod model_tests;
