use super::WebService;
use crate::model::item::{Item, ItemMac};
use crate::model::{Database, Db};
use crate::web::warp::with_db;
use async_trait::async_trait;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub struct ItemService;

#[async_trait]
impl WebService<String, Item> for ItemService {
    async fn list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
        let items = ItemMac::list(&db).await?;
        Self::json_response(items)
    }

    async fn get(db: Arc<Db>, name: String) -> Result<Json, warp::Rejection> {
        let name = name.replace("%20", " ");
        let item = ItemMac::get(&db, name).await?;
        Self::json_response(item)
    }

    async fn create(db: Arc<Db>, patch: Item) -> Result<Json, warp::Rejection> {
        let item = ItemMac::create(&db, patch).await?;
        Self::json_response(item)
    }

    async fn update(db: Arc<Db>, name: String, patch: Item) -> Result<Json, warp::Rejection> {
        let name = name.replace("%20", " ");
        let item = ItemMac::update(&db, name, patch).await?;
        Self::json_response(item)
    }

    async fn delete(db: Arc<Db>, name: String) -> Result<Json, warp::Rejection> {
        let name = name.replace("%20", " ");
        let item = ItemMac::delete(&db, name).await?;
        Self::json_response(item)
    }
}

pub fn item_rest_filters(
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let items_path = warp::path("api").and(warp::path("items"));
    let common = with_db(db);

    // list items `GET items/`
    let list = items_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(ItemService::list);

    // get item `GET /items/100`
    let get = items_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(ItemService::get);

    // create item `POST /items with body itemPatch`
    let create = items_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(ItemService::create);

    // update item `PATCH /items/100 with body itemPatch`
    let update = items_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(ItemService::update);

    // delete item `DELETE /items/100`
    let delete = items_path
        .and(warp::delete())
        .and(common)
        .and(warp::path::param())
        .and_then(ItemService::delete);

    list.or(get).or(create).or(update).or(delete)
}

#[cfg(test)]
#[path = "../../tests/web_warp_item.rs"]
mod tests;
