use crate::model::{Database, Db, PurchaseMac, PurchasePatch};
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;
use std::sync::Arc;

#[get("/api/purchases")]
pub async fn list(db: Data<Arc<Db>>) -> HttpResponse {
    let purchases = PurchaseMac::list(&db).await.unwrap();

    HttpResponse::Ok().json(purchases)
}

#[get("/api/purchases/{id}")]
pub async fn get(db: Data<Arc<Db>>, id: String) -> HttpResponse {
    let id = id.parse::<i64>().unwrap();
    let purchase = PurchaseMac::get(&db, id).await.unwrap();

    HttpResponse::Ok().json(purchase)
}

#[post("/api/purchases")]
pub async fn create(db: Data<Arc<Db>>, purchase: Json<PurchasePatch>) -> HttpResponse {
    let purchase = purchase.into_inner();
    let purchase = PurchaseMac::create(&db, purchase).await.unwrap();

    HttpResponse::Ok().json(purchase)
}

#[put("/api/purchases/{id}")]
pub async fn update(db: Data<Arc<Db>>, id: String, purchase: Json<PurchasePatch>) -> HttpResponse {
    let id = id.parse::<i64>().unwrap();
    let purchase = purchase.into_inner();
    let purchase = PurchaseMac::update(&db, id, purchase).await.unwrap();

    HttpResponse::Ok().json(purchase)
}

#[delete("/api/purchases/{id}")]
pub async fn delete(db: Data<Arc<Db>>, id: String) -> HttpResponse {
    let id = id.parse::<i64>().unwrap();
    let purchase = PurchaseMac::delete(&db, id).await.unwrap();

    HttpResponse::Ok().json(purchase)
}

#[cfg(test)]
#[path = "../../../tests/web_tests/actix_tests/purchase.rs"]
mod actix_tests;
