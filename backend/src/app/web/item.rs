use crate::app::model::{item::Item, DbPool};
use crate::app::web::handle_result;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;

#[post("/api/items")]
pub async fn create(db: Data<DbPool>, item: Json<Item>) -> HttpResponse {
    let db = db.get().unwrap();
    let item = item.into_inner();
    let item = Item::create(&db, item);

    handle_result(item)
}

#[get("/api/items")]
pub async fn list(db: Data<DbPool>) -> HttpResponse {
    let db = db.get().unwrap();
    let items = Item::list(&db);

    handle_result(items)
}

#[get("/api/items/{name}")]
pub async fn get(db: Data<DbPool>, name: Path<String>) -> HttpResponse {
    let db = db.get().unwrap();
    let item = Item::get_by_name(&db, name.to_string());

    handle_result(item)
}

#[put("/api/items/{name}")]
pub async fn update(db: Data<DbPool>, name: Path<String>, item: Json<Item>) -> HttpResponse {
    let name = name.replace("%20", " ");
    let db = db.get().unwrap();
    let item = item.into_inner();
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
    cfg.service(get);
    cfg.service(update);
    cfg.service(delete);
}
