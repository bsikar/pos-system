use super::WebService;
use crate::model::item::{Item, ItemMac};
use crate::model::{Database, Db};
use crate::web::with_db;
use async_trait::async_trait;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub struct ItemService;

#[async_trait]
impl WebService<String, Item> for ItemService {
    async fn list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
        let purchases = ItemMac::list(&db).await?;
        Self::json_response(purchases)
    }

    async fn get(db: Arc<Db>, name: String) -> Result<Json, warp::Rejection> {
        let purchase = ItemMac::get(&db, name).await?;
        Self::json_response(purchase)
    }

    async fn create(db: Arc<Db>, patch: Item) -> Result<Json, warp::Rejection> {
        let purchase = ItemMac::create(&db, patch).await?;
        Self::json_response(purchase)
    }

    async fn update(db: Arc<Db>, name: String, patch: Item) -> Result<Json, warp::Rejection> {
        let purchase = ItemMac::update(&db, name, patch).await?;
        Self::json_response(purchase)
    }

    async fn delete(db: Arc<Db>, name: String) -> Result<Json, warp::Rejection> {
        let purchase = ItemMac::delete(&db, name).await?;
        Self::json_response(purchase)
    }
}

pub fn item_rest_filters(
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let purchases_path = warp::path("api").and(warp::path("items"));
    let common = with_db(db);

    // list purchases `GET purchases/`
    let list = purchases_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(ItemService::list);

    // get purchase `GET /purchases/100`
    let get = purchases_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(ItemService::get);

    // create purchase `POST /purchases with body purchasePatch`
    let create = purchases_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(ItemService::create);

    // update purchase `PATCH /purchases/100 with body purchasePatch`
    let update = purchases_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(ItemService::update);

    // delete purchase `DELETE /purchases/100`
    let delete = purchases_path
        .and(warp::delete())
        .and(common)
        .and(warp::path::param())
        .and_then(ItemService::delete);

    list.or(get).or(create).or(update).or(delete)
}
