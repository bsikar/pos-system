use crate::model::item::{Item, ItemMac};
use crate::{model::Database, web::Db};
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use std::sync::Arc;

#[get("/api/items")]
pub async fn list(db: Data<Arc<Db>>) -> HttpResponse {
    let items = ItemMac::list(&db).await.unwrap();

    HttpResponse::Ok().json(items)
}

#[get("/api/items/{name}")]
pub async fn get(db: Data<Arc<Db>>, name: String) -> HttpResponse {
    let name = name.replace("%20", " ");
    let item = ItemMac::get(&db, name).await.unwrap();

    HttpResponse::Ok().json(item)
}

#[post("/api/items")]
pub async fn create(db: Data<Arc<Db>>, item: Json<Item>) -> HttpResponse {
    let item = item.into_inner();
    let item = ItemMac::create(&db, item).await.unwrap();

    HttpResponse::Ok().json(item)
}

#[put("/api/items/{name}")]
pub async fn update(db: Data<Arc<Db>>, name: String, item: Json<Item>) -> HttpResponse {
    let name = name.replace("%20", " ");
    let item = item.into_inner();
    let item = ItemMac::update(&db, name, item).await.unwrap();

    HttpResponse::Ok().json(item)
}

#[delete("/api/items/{name}")]
pub async fn delete(db: Data<Arc<Db>>, name: String) -> HttpResponse {
    let name = name.replace("%20", " ");
    let item = ItemMac::delete(&db, name).await.unwrap();

    HttpResponse::Ok().json(item)
}

pub fn item_rest_filters(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(list);
    cfg.service(get);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}

#[cfg(test)]
#[path = "../../../tests/web_tests/actix_tests/item.rs"]
mod actix_tests;
