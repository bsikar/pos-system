use crate::app::model::{purchase::Purchase, DbPool};
use crate::app::web::handle_result;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use serde_json::Value as JsonValue;

#[get("/api/purchases")]
pub async fn list(db: Data<DbPool>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchases = Purchase::list(&db);

    handle_result(purchases)
}

#[get("/api/purchases/{id}")]
pub async fn get(db: Data<DbPool>, id: Path<i64>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchase = Purchase::get_by_id(&db, *id);

    handle_result(purchase)
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
pub async fn update(db: Data<DbPool>, id: Path<i64>, purchase: Json<JsonValue>) -> HttpResponse {
    let db = db.get().unwrap();
    let purchase = purchase.into_inner();
    let purchase = Purchase::update(&db, *id, purchase);

    handle_result(purchase)
}

#[delete("/api/purchases/{id}")]
pub async fn delete(db: Data<DbPool>, id: Path<i64>) -> HttpResponse {
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
