use crate::model;
use crate::model::Db;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: i64,
}

impl Item {
    pub fn new(name: String, price: i64) -> Self {
        Self { name, price }
    }

    pub async fn validate(&self, db: &Db) -> Result<(), model::Error> {
        // check if item is in database
        let result = ItemMac::get_by_name(db, self.name.clone()).await;

        if result.is_err() {
            Err(model::Error::InvalidItemName(self.name.clone()))
        } else if result.unwrap().price != self.price {
            Err(model::Error::InvalidItemPrice(self.price))
        } else {
            Ok(())
        }
    }
}

// Mac: model access controller
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
            return Err(model::Error::InvalidItemName(data.name));
        }

        let sql = r#"INSERT INTO item ("name", price) VALUES ($1, $2) RETURNING "name", price"#;
        let query = sqlx::query_as(sql).bind(data.name).bind(data.price);

        let item = query.fetch_one(db).await?;

        Ok(item)
    }

    async fn get(db: &Db, name: String) -> Result<Item, model::Error> {
        let sql = r#"SELECT "name", price FROM item WHERE "name" = $1"#;
        let query = sqlx::query_as(sql).bind(&name);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "item", name)
    }

    async fn update(db: &Db, name: String, data: Item) -> Result<Item, model::Error> {
        let sql =
            r#"UPDATE item SET "name" = $1, price = $2 WHERE "name" = $3 RETURNING "name", price"#;
        let query = sqlx::query_as(sql)
            .bind(data.name)
            .bind(data.price)
            .bind(&name);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "item", name)
    }

    async fn list(db: &Db) -> Result<Vec<Item>, model::Error> {
        let sql = r#"SELECT "name", price FROM item"#;
        let query = sqlx::query_as(sql);

        let items = query.fetch_all(db).await?;

        Ok(items)
    }

    async fn delete(db: &Db, name: String) -> Result<Item, model::Error> {
        let sql = r#"DELETE FROM item WHERE "name" = $1 RETURNING "name", price"#;
        let query = sqlx::query_as(sql).bind(&name);

        let result = query.fetch_one(db).await;

        Self::handle_fetch_one_result(result, "item", name)
    }
}

#[cfg(test)]
#[path = "../tests/model_item.rs"]
mod tests;
