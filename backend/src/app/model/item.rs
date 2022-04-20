use crate::app::model::Error as ModelError;
use crate::schema::items::{self, dsl};
use diesel::associations::HasTable;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Queryable, QueryableByName, Insertable, AsChangeset, Debug, Deserialize, Serialize)]
#[table_name = "items"]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub tax: f32,
    pub type_: String,
}

impl Item {
    pub fn new(name: String, price: i32, tax: f32, type_: String) -> Self {
        Item {
            name,
            price,
            tax,
            type_,
        }
    }

    pub fn get_by_name(db: &SqliteConnection, name: String) -> Result<Item, ModelError> {
        dsl::items
            .filter(dsl::name.eq(name.clone()))
            .first::<Item>(db)
            .map_err(|_| ModelError::ItemNotFound(name))
    }

    pub fn is_food(&self) -> bool {
        self.type_ == "food"
    }

    pub fn is_drink(&self) -> bool {
        self.type_ == "drink"
    }

    pub fn is_other(&self) -> bool {
        self.type_ == "other"
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

    pub fn from_json(data: JsonValue) -> Item {
        Item::new(
            data["name"].as_str().unwrap().to_string(),
            data["price"].as_i64().unwrap() as i32,
            data["tax"].as_f64().unwrap() as f32,
            data["type"].as_str().unwrap().to_string(),
        )
    }

    pub fn create_from_json(db: &SqliteConnection, data: JsonValue) -> Result<Item, ModelError> {
        let data = Item::from_json(data);
        Item::create(db, data)
    }

    pub fn create(db: &SqliteConnection, data: Item) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, data.name.clone());

        if item.is_ok() {
            return Err(ModelError::ItemAlreadyExists(data.name));
        }

        if data.price < 0 {
            return Err(ModelError::InvalidItemPrice(data.price));
        }

        if data.tax < 0.0 {
            return Err(ModelError::InvalidItemTax(data.tax));
        }

        if data.name.is_empty() {
            return Err(ModelError::EmptyItemName);
        }

        match data.type_.to_ascii_lowercase().as_str() {
            "food" | "drink" | "other" => {}
            _ => {
                return Err(ModelError::InvalidItemType(data.type_));
            }
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
