use crate::app::model::{self, Db};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Error::RowNotFound;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: i64,
    pub tax: f32,
}

impl Item {
    pub fn new(name: String, price: i64, tax: f32) -> Self {
        Self { name, price, tax }
    }

    pub async fn validate(&self, db: &Db) -> Result<(), model::Error> {
        // check if item is in database
        let result = ItemMac::get_by_name(db, self.name.clone()).await;

        if let Err(e) = result {
            if matches!(e, model::Error::SqlxError(RowNotFound)) {
                Err(model::Error::InvalidItemName(self.name.clone()))
            } else {
                Err(e)
            }
        } else if result.unwrap().price != self.price {
            Err(model::Error::InvalidItemPrice(self.price))
        } else {
            Ok(())
        }
    }
}

// Mac: model access controller
#[derive(Deserialize)]
pub struct ItemMac;

impl ItemMac {
    pub async fn get_by_name(db: &Db, name: String) -> Result<Item, model::Error> {
        let sql = "SELECT * FROM item WHERE name = $1";
        let query = sqlx::query_as(sql).bind(name);

        let item = query.fetch_one(db).await?;

        Ok(item)
    }
}

#[async_trait]
impl model::Database<Item, Item, String> for ItemMac {
    async fn create(db: &Db, data: Item) -> Result<Item, model::Error> {
        // check if item already exists
        let item = ItemMac::get_by_name(db, data.name.clone()).await;
        if item.is_ok() {
            return Err(model::Error::ItemAlreadyExists(data.name));
        }

        if data.price < 0 {
            return Err(model::Error::InvalidItemPrice(data.price));
        }

        if data.name.is_empty() {
            return Err(model::Error::EmptyItemName);
        }

        let sql = r#"INSERT INTO item ("name", price, tax) VALUES ($1, $2, $3) RETURNING "name", price, tax"#;
        let query = sqlx::query_as(sql)
            .bind(data.name)
            .bind(data.price)
            .bind(data.tax);

        let item = query.fetch_one(db).await?;

        Ok(item)
    }

    async fn get(db: &Db, name: String) -> Result<Item, model::Error> {
        let sql = r#"SELECT "name", price, tax FROM item WHERE "name" = $1"#;
        let query = sqlx::query_as(sql).bind(&name);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "item", name)
    }

    async fn update(db: &Db, name: String, data: Item) -> Result<Item, model::Error> {
        let sql = r#"UPDATE item SET "name" = $1, price = $2, tax = $3 WHERE "name" = $4 RETURNING "name", price, tax"#;
        let query = sqlx::query_as(sql)
            .bind(data.name)
            .bind(data.price)
            .bind(data.tax)
            .bind(&name);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "item", name)
    }

    async fn list(db: &Db) -> Result<Vec<Item>, model::Error> {
        let sql = r#"SELECT "name", price, tax FROM item"#;
        let query = sqlx::query_as(sql);

        let items = query.fetch_all(db).await?;

        Ok(items)
    }

    async fn delete(db: &Db, name: String) -> Result<Item, model::Error> {
        let sql = r#"DELETE FROM item WHERE "name" = $1 RETURNING "name", price, tax"#;
        let query = sqlx::query_as(sql).bind(&name);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "item", name)
    }
}

#[cfg(test)]
#[path = "../../../tests/model_tests/item.rs"]
mod model_tests;
