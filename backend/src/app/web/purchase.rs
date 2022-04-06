use crate::app::model::{purchase::Purchase, DbPool};
use crate::app::web::handle_result;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseMac {
    pub id: i32,
    pub ctime: String,
    pub items: JsonValue,
    pub total: i32,
}

#[get("/api/purchases")]
pub async fn list(db: Data<DbPool>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchases = Purchase::list(&db);
    if purchases.is_err() {
        return handle_result(purchases);
    }

    let purchases = purchases
        .unwrap()
        .iter()
        .map(|p| p.to_json())
        .collect::<Vec<_>>();

    handle_result(Ok(purchases))
}

#[get("/api/purchases/{id}")]
pub async fn get(db: Data<DbPool>, id: Path<i32>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchase = Purchase::get_by_id(&db, *id);
    if purchase.is_err() {
        return handle_result(purchase);
    }

    let purchase = purchase.unwrap().to_json();

    handle_result(Ok(purchase))
}

#[post("/api/purchases")]
pub async fn create(db: Data<DbPool>, purchase: Json<JsonValue>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchase = purchase.into_inner();
    let purchase = purchase["items"].clone();
    let purchase = Purchase::create(&db, purchase);

    handle_result(purchase)
}

#[put("/api/purchases/{id}")]
pub async fn update(db: Data<DbPool>, id: Path<i32>, purchase: Json<JsonValue>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchase = purchase.into_inner();
    let purchase = Purchase::update(&db, *id, purchase);

    handle_result(purchase)
}

#[delete("/api/purchases/{id}")]
pub async fn delete(db: Data<DbPool>, id: Path<i32>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchase = Purchase::delete(&db, *id);

    handle_result(purchase)
}

pub fn purchase_rest_filters(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(list);
    cfg.service(get);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}

#[cfg(test)]
#[path = "../../../tests/web_tests/purchase.rs"]
mod web_tests;
