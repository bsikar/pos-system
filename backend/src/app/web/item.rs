use crate::app::model::{item::Item, DbPool};
use crate::app::web::handle_result;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use serde_json::Value as JsonValue;

#[post("/api/items")]
pub async fn create(db: Data<DbPool>, item: Json<JsonValue>) -> HttpResponse {
    let db = db.get().unwrap();
    let item = item.into_inner();
    let item = Item::create_from_json(&db, item);

    handle_result(item)
}

#[get("/api/items")]
pub async fn list(db: Data<DbPool>) -> HttpResponse {
    let db = db.get().unwrap();
    let items = Item::list(&db);

    handle_result(items)
}

#[get("/api/items/food")]
pub async fn list_food(db: Data<DbPool>) -> HttpResponse {
    let db = db.get().unwrap();
    let items = Item::list(&db);

    handle_result(items.map(|items| {
        items
            .into_iter()
            .filter(|item| item.is_food())
            .collect::<Vec<_>>()
    }))
}

#[get("/api/items/drinks")]
pub async fn list_drink(db: Data<DbPool>) -> HttpResponse {
    let db = db.get().unwrap();
    let items = Item::list(&db);

    handle_result(items.map(|items| {
        items
            .into_iter()
            .filter(|item| item.is_drink())
            .collect::<Vec<_>>()
    }))
}

#[get("/api/items/other")]
pub async fn list_other(db: Data<DbPool>) -> HttpResponse {
    let db = db.get().unwrap();
    let items = Item::list(&db);

    handle_result(items.map(|items| {
        items
            .into_iter()
            .filter(|item| item.is_other())
            .collect::<Vec<_>>()
    }))
}

#[get("/api/items/name/{name}")]
pub async fn get(db: Data<DbPool>, name: Path<String>) -> HttpResponse {
    let db = db.get().unwrap();
    let item = Item::get_by_name(&db, name.to_string());

    handle_result(item)
}

#[put("/api/items/{name}")]
pub async fn update(db: Data<DbPool>, name: Path<String>, item: Json<JsonValue>) -> HttpResponse {
    let name = name.replace("%20", " ");
    let db = db.get().unwrap();
    let item = item.into_inner();
    let item = Item::from_json(item);
    let item = Item::update(&db, name, item);

    handle_result(item)
}

#[delete("/api/items/{name}")]
pub async fn delete(db: Data<DbPool>, name: Path<String>) -> HttpResponse {
    let name = name.replace("%20", " ");
    let db = db.get().unwrap();
    let item = Item::delete(&db, name);

    handle_result(item)
}

pub fn item_rest_filters(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create);
    cfg.service(list);
    cfg.service(list_food);
    cfg.service(list_drink);
    cfg.service(list_other);
    cfg.service(get);
    cfg.service(update);
    cfg.service(delete);
}

#[cfg(test)]
#[path = "../../../tests/web_tests/item.rs"]
mod web_tests;
