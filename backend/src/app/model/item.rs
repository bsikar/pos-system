use crate::app::model::Error as ModelError;
use crate::schema::items::{self, dsl};
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, QueryableByName, Insertable, AsChangeset, Debug, Deserialize, Serialize)]
#[table_name = "items"]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub tax: f32,
}

impl Item {
    pub fn new(name: String, price: i32, tax: f32) -> Self {
        Item { name, price, tax }
    }

    pub fn get_by_name(db: &SqliteConnection, name: String) -> Result<Item, ModelError> {
        dsl::items
            .filter(dsl::name.eq(name.clone()))
            .first::<Item>(db)
            .map_err(|_| ModelError::ItemNotFound(name))
    }

    pub fn validate(&self, db: &SqliteConnection) -> Result<(), ModelError> {
        let result = Item::get_by_name(db, self.name.clone());

        if let Err(err) = result {
            Err(err)
        } else if result.unwrap().price != self.price {
            Err(ModelError::InvalidItemPrice(self.price))
        } else {
            Ok(())
        }
    }

    pub fn create(db: &SqliteConnection, data: Item) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, data.name.clone());

        if item.is_ok() {
            return Err(ModelError::ItemAlreadyExists(data.name));
        }

        if data.price < 0 {
            return Err(ModelError::InvalidItemPrice(data.price));
        }

        if data.name.is_empty() {
            return Err(ModelError::EmptyItemName);
        }

        diesel::insert_into(dsl::items)
            .values(&data)
            .execute(db)
            .map_or_else(|e| Err(ModelError::DieselError(e)), |_| Ok(data))
    }

    pub fn list(db: &SqliteConnection) -> Result<Vec<Item>, ModelError> {
        match dsl::items.load::<Item>(db) {
            Ok(items) => Ok(items),
            Err(e) => Err(ModelError::DieselError(e)),
        }
    }

    pub fn get(db: &SqliteConnection, data: Item) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, data.name);

        if let Err(err) = item {
            return Err(err);
        }

        Ok(item.unwrap())
    }

    pub fn update(db: &SqliteConnection, name: String, data: Item) -> Result<Item, ModelError> {
        // set the current item with the name `name` to the data
        let item = Item::get_by_name(db, name.clone());

        if let Err(err) = item {
            return Err(err);
        }

        if data.price < 0 {
            return Err(ModelError::InvalidItemPrice(data.price));
        }

        if data.name.is_empty() {
            return Err(ModelError::EmptyItemName);
        }

        diesel::update(dsl::items::table().find(&name))
            .set(&data)
            .execute(db)
            .map_or_else(|e| Err(ModelError::DieselError(e)), |_| Ok(data))
    }

    pub fn delete(db: &SqliteConnection, name: String) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, name.clone());

        if let Err(err) = item {
            return Err(err);
        }

        diesel::delete(dsl::items::table().find(&name))
            .execute(db)
            .map_or_else(|e| Err(ModelError::DieselError(e)), |_| item)
    }
}

#[cfg(test)]
#[path = "../../../tests/model_tests/item.rs"]
mod model_tests;
