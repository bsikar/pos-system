use crate::app::model::Error as ModelError;
use crate::schema::items::{self, dsl};
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, RunQueryDsl};

#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "items"]
pub struct Item {
    pub name: String,
    pub price: i64,
    pub tax: f32,
}

impl Item {
    pub fn new(name: String, price: i64, tax: f32) -> Self {
        Item { name, price, tax }
    }

    pub fn get_by_name(db: &PgConnection, name: String) -> Option<Item> {
        dsl::items.filter(dsl::name.eq(name)).first::<Item>(db).ok()
    }

    pub fn validate(&self, db: &PgConnection) -> Result<(), ModelError> {
        let result = Item::get_by_name(db, self.name.clone());

        if result.is_none() {
            Err(ModelError::ItemNotFound(self.name.clone()))
        } else if result.unwrap().price != self.price {
            Err(ModelError::InvalidItemPrice(self.price))
        } else {
            Ok(())
        }
    }

    pub fn create(db: &PgConnection, data: Item) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, data.name.clone());

        if item.is_some() {
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

    pub fn list(db: &PgConnection) -> Result<Vec<Item>, ModelError> {
        Ok(dsl::items.load::<Item>(db).unwrap())
    }

    pub fn update(db: &PgConnection, data: Item) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, data.name.clone());

        if item.is_none() {
            return Err(ModelError::ItemNotFound(data.name));
        }

        if data.price < 0 {
            return Err(ModelError::InvalidItemPrice(data.price));
        }

        if data.name.is_empty() {
            return Err(ModelError::EmptyItemName);
        }

        diesel::update(dsl::items::table().find(&data.name))
            .set(&data)
            .execute(db)
            .map_or_else(|e| Err(ModelError::DieselError(e)), |_| Ok(data))
    }

    pub fn delete(db: &PgConnection, data: Item) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, data.name.clone());

        if item.is_none() {
            return Err(ModelError::ItemNotFound(data.name));
        }

        diesel::delete(dsl::items::table().find(&data.name))
            .execute(db)
            .map_or_else(|e| Err(ModelError::DieselError(e)), |_| Ok(data))
    }

    pub fn get(db: &PgConnection, data: Item) -> Result<Item, ModelError> {
        let item = Item::get_by_name(db, data.name.clone());

        if item.is_none() {
            return Err(ModelError::ItemNotFound(data.name));
        }

        Ok(item.unwrap())
    }
}
