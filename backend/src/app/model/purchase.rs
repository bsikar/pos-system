use crate::app::model::item::Item;
use crate::app::model::Error as ModelError;
use crate::schema::purchases::{self, dsl};
use chrono::Local;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

#[derive(Queryable, QueryableByName, Insertable, AsChangeset, Debug, Deserialize, Serialize)]
#[table_name = "purchases"]
pub struct Purchase {
    pub id: i32,
    pub ctime: String,
    pub items: String,
    pub total: i32,
}

impl Purchase {
    pub fn create(db: &SqliteConnection, data: JsonValue) -> Result<Purchase, ModelError> {
        Purchase::validate(&data, db)?;

        let time = Local::now().naive_local();
        let total = Purchase::calculate_total(&data);
        let data = format!("{}", data);
        let time = format!("{}", time);

        let result = diesel::insert_into(dsl::purchases)
            .values((
                dsl::items.eq(data),
                dsl::ctime.eq(time),
                dsl::total.eq(total),
            ))
            .execute(db);

        if let Err(e) = result {
            Err(ModelError::DieselError(e))
        } else {
            Ok(Purchase::get_last_purchase(db).unwrap())
        }
    }

    pub fn get_by_id(db: &SqliteConnection, id: i32) -> Result<Purchase, ModelError> {
        dsl::purchases
            .find(id)
            .first::<Purchase>(db)
            .map_or_else(|_| Err(ModelError::PurchaseNotFound(id)), Ok)
    }

    pub fn get_last_purchase(db: &SqliteConnection) -> Result<Purchase, ModelError> {
        let result = dsl::purchases.order(dsl::id.desc()).first::<Purchase>(db);

        if let Err(e) = result {
            Err(ModelError::DieselError(e))
        } else {
            Ok(result.unwrap())
        }
    }

    pub fn list(db: &SqliteConnection) -> Result<Vec<Purchase>, ModelError> {
        match dsl::purchases.load::<Purchase>(db) {
            Ok(purchases) => Ok(purchases),
            Err(e) => Err(ModelError::DieselError(e)),
        }
    }

    pub fn update(db: &SqliteConnection, id: i32, data: JsonValue) -> Result<Purchase, ModelError> {
        Purchase::validate(&data, db)?;

        let total = Purchase::calculate_total(&data);
        let data = format!("{}", data);

        let result = diesel::update(dsl::purchases.filter(dsl::id.eq(id)))
            .set((dsl::items.eq(data), dsl::total.eq(total)))
            .execute(db);

        if let Err(e) = result {
            Err(ModelError::DieselError(e))
        } else {
            Ok(Purchase::get_by_id(db, id).unwrap())
        }
    }

    pub fn delete(db: &SqliteConnection, id: i32) -> Result<Purchase, ModelError> {
        let purchase = Purchase::get_by_id(db, id)?;
        let result = diesel::delete(dsl::purchases.filter(dsl::id.eq(id))).execute(db);

        if let Err(e) = result {
            Err(ModelError::DieselError(e))
        } else {
            Ok(purchase)
        }
    }

    pub fn calculate_total(data: &JsonValue) -> i32 {
        let mut total = 0;

        if let Some(values) = data.as_array() {
            for item in values {
                let price = item["price"].as_i64().unwrap();
                let quantity = item["quantity"].as_i64().unwrap();
                let tax_percent = 1.0 + item["tax"].as_f64().unwrap() as f32 / 100.0;

                let mut subtotal = price as f32 / 100.0 * quantity as f32;
                subtotal *= tax_percent;

                let subtotal = format!("{:.2}", subtotal);
                let subtotal: i32 = subtotal.replace('.', "").parse().unwrap();

                total += subtotal;
            }
        }

        total
    }

    pub fn ctime_to_ndt(&self) -> NaiveDateTime {
        let fmt = "%Y-%m-%d %H:%M:%S%.f";
        NaiveDateTime::parse_from_str(&self.ctime, fmt).unwrap()
    }

    pub fn items_to_json(&self) -> JsonValue {
        serde_json::from_str(&self.items).unwrap()
    }

    pub fn to_json(&self) -> JsonValue {
        json!({
            "id": self.id,
            "ctime": self.ctime,
            "items": self.items_to_json(),
            "total": self.total,
        })
    }

    fn to_items(data: JsonValue) -> Result<Vec<Item>, ModelError> {
        let mut items = vec![];

        if data == json!(null) || data == json!({}) || data == json!([]) {
            return Err(ModelError::EmptyItems);
        }

        for item in data.as_array().unwrap() {
            let name = item["name"].as_str().unwrap().to_string();
            let price = item["price"].as_i64().unwrap() as i32;
            let tax = item["tax"].as_f64().unwrap() as f32;

            items.push(Item::new(name, price, tax));
        }

        Ok(items)
    }

    fn validate(data: &JsonValue, db: &SqliteConnection) -> Result<(), ModelError> {
        let items = Purchase::to_items(data.clone())?;

        for item in items {
            item.validate(db)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "../../../tests/model_tests/purchase.rs"]
mod model_tests;
