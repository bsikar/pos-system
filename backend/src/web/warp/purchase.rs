use super::WebService;
use crate::model::{Database, Db, PurchaseMac, PurchasePatch};
use crate::web::warp::with_db;
use async_trait::async_trait;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub struct PurchaseService;

#[async_trait]
impl WebService<i64, PurchasePatch> for PurchaseService {
    async fn list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
        let purchases = PurchaseMac::list(&db).await?;
        Self::json_response(purchases)
    }

    async fn get(db: Arc<Db>, id: i64) -> Result<Json, warp::Rejection> {
        let purchase = PurchaseMac::get(&db, id).await?;
        Self::json_response(purchase)
    }

    async fn create(db: Arc<Db>, patch: PurchasePatch) -> Result<Json, warp::Rejection> {
        let purchase = PurchaseMac::create(&db, patch).await?;
        Self::json_response(purchase)
    }

    async fn update(db: Arc<Db>, id: i64, patch: PurchasePatch) -> Result<Json, warp::Rejection> {
        let purchase = PurchaseMac::update(&db, id, patch).await?;
        Self::json_response(purchase)
    }

    async fn delete(db: Arc<Db>, id: i64) -> Result<Json, warp::Rejection> {
        let purchase = PurchaseMac::delete(&db, id).await?;
        Self::json_response(purchase)
    }
}

pub fn purchase_rest_filters(
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let purchases_path = warp::path("api").and(warp::path("purchases"));
    let common = with_db(db);

    // list purchases `GET purchases/`
    let list = purchases_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(PurchaseService::list);

    // get purchase `GET /purchases/100`
    let get = purchases_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(PurchaseService::get);

    // create purchase `POST /purchases with body purchasePatch`
    let create = purchases_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(PurchaseService::create);

    // update purchase `PATCH /purchases/100 with body purchasePatch`
    let update = purchases_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(PurchaseService::update);

    // delete purchase `DELETE /purchases/100`
    let delete = purchases_path
        .and(warp::delete())
        .and(common)
        .and(warp::path::param())
        .and_then(PurchaseService::delete);

    list.or(get).or(create).or(update).or(delete)
}

#[cfg(test)]
#[path = "../../tests/web_warp_purchase.rs"]
mod tests;
