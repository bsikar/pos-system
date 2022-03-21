use super::handle_result;
use crate::model::item::{Item, ItemMac};
use crate::model::{Database, Db};
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;

#[get("/api/items")]
pub async fn list(db: Data<Db>) -> HttpResponse {
    let items = ItemMac::list(&db).await;

    handle_result(items)
}

#[get("/api/items/{name}")]
pub async fn get(db: Data<Db>, name: Path<String>) -> HttpResponse {
    let name = name.replace("%20", " ");
    let item = ItemMac::get(&db, name).await;

    handle_result(item)
}

#[post("/api/items")]
pub async fn create(db: Data<Db>, item: Json<Item>) -> HttpResponse {
    let item = item.into_inner();
    let item = ItemMac::create(&db, item).await;

    handle_result(item)
}

#[put("/api/items/{name}")]
pub async fn update(db: Data<Db>, name: Path<String>, item: Json<Item>) -> HttpResponse {
    let name = name.replace("%20", " ");
    let item = item.into_inner();
    let item = ItemMac::update(&db, name, item).await;

    handle_result(item)
}

#[delete("/api/items/{name}")]
pub async fn delete(db: Data<Db>, name: Path<String>) -> HttpResponse {
    let name = name.replace("%20", " ");
    let item = ItemMac::delete(&db, name).await;

    handle_result(item)
}

pub fn item_rest_filters(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(list);
    cfg.service(get);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}

#[cfg(test)]
#[path = "../../tests/web_tests/item.rs"]
mod web_tests;
