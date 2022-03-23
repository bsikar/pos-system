use crate::app::model::item::Item;
use crate::app::model::Error as ModelError;
use crate::schema::purchases::{self, dsl};
use chrono::{Local, NaiveDateTime};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use serde_json::{json, Value as JsonValue};

#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "purchases"]
pub struct Purchase {
    pub id: i64,
    pub ctime: NaiveDateTime,
    pub items: JsonValue,
    pub total: i64,
}

impl Purchase {
    fn to_items(data: JsonValue) -> Result<Vec<Item>, ModelError> {
        let mut items = vec![];

        if data == json!(null) || data == json!({}) || data == json!([]) {
            return Err(ModelError::EmptyItems);
        }

        for item in data.as_array().unwrap() {
            let name = item["name"].as_str().unwrap().to_string();
            let price = item["price"].as_i64().unwrap();
            let tax = item["tax"].as_f64().unwrap() as f32;

            items.push(Item::new(name, price, tax));
        }

        Ok(items)
    }

    fn validate(data: &JsonValue, db: &PgConnection) -> Result<(), ModelError> {
        let items = Purchase::to_items(data.clone())?;

        for item in items {
            item.validate(db)?;
        }

        Ok(())
    }

    pub fn create(db: &PgConnection, data: JsonValue) -> Result<Purchase, ModelError> {
        Purchase::validate(&data, db)?;

        let time = Local::now().naive_local();
        let total = Purchase::calculate_total(&data);

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
            //Ok(Purchase::get_last_purchase(db).unwrap())
            Ok(Purchase::list(&db).unwrap().pop().unwrap())
        }
    }

    pub fn list(db: &PgConnection) -> Result<Vec<Purchase>, ModelError> {
        Ok(dsl::purchases.load::<Purchase>(db).unwrap())
    }

    pub fn calculate_total(data: &JsonValue) -> i64 {
        let mut total = 0;

        if let Some(values) = data.as_array() {
            for item in values {
                let price = item["price"].as_i64().unwrap();
                let quantity = item["quantity"].as_i64().unwrap();
                let tax = item["tax"].as_f64().unwrap() as f32;

                total += ((price * quantity) as f32 * tax) as i64;
            }
        }

        total
    }
}
