use super::handle_result;
use crate::model::{Database, Db, PurchaseMac, PurchasePatch};
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;

#[get("/api/purchases")]
pub async fn list(db: Data<Db>) -> HttpResponse {
    let purchases = PurchaseMac::list(&db).await;

    handle_result(purchases)
}

#[get("/api/purchases/{id}")]
pub async fn get(db: Data<Db>, id: String) -> HttpResponse {
    let id = id.parse::<i64>().unwrap();
    let purchase = PurchaseMac::get(&db, id).await;

    handle_result(purchase)
}

#[post("/api/purchases")]
pub async fn create(db: Data<Db>, purchase: Json<PurchasePatch>) -> HttpResponse {
    let purchase = purchase.into_inner();
    let purchase = PurchaseMac::create(&db, purchase).await;

    handle_result(purchase)
}

#[put("/api/purchases/{id}")]
pub async fn update(db: Data<Db>, id: String, purchase: Json<PurchasePatch>) -> HttpResponse {
    let id = id.parse::<i64>().unwrap();
    let purchase = purchase.into_inner();
    let purchase = PurchaseMac::update(&db, id, purchase).await;

    handle_result(purchase)
}

#[delete("/api/purchases/{id}")]
pub async fn delete(db: Data<Db>, id: String) -> HttpResponse {
    let id = id.parse::<i64>().unwrap();
    let purchase = PurchaseMac::delete(&db, id).await;

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
#[path = "../../../tests/web_tests/actix_tests/purchase.rs"]
mod actix_tests;
