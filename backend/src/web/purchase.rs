use crate::model::{Db, PurchaseMac, PurchasePatch};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn purchase_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let purchases_path = warp::path(base_path).and(warp::path("purchases"));
    let common = super::filter_util::with_db(db);

    // LIST purchases `GET purchases/`
    let list = purchases_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(purchase_list);

    // GET purchase `GET /purchases/100`
    let get = purchases_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(purchase_get);

    // CREATE purchase `POST /purchases with body purchasePatch`
    let create = purchases_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(purchase_create);

    // UPDATE purchase `PATCH /purchases/100 with body purchasePatch`
    let update = purchases_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(purchase_update);

    // DELETE purchase `DELETE /purchases/100`
    let delete = purchases_path
        .and(warp::delete())
        .and(common)
        .and(warp::path::param())
        .and_then(purchase_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn purchase_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    let purchases = PurchaseMac::list(&db).await?;
    json_response(purchases)
}

async fn purchase_get(db: Arc<Db>, id: i64) -> Result<Json, warp::Rejection> {
    let purchase = PurchaseMac::get(&db, id).await?;
    json_response(purchase)
}

async fn purchase_create(db: Arc<Db>, patch: PurchasePatch) -> Result<Json, warp::Rejection> {
    let purchase = PurchaseMac::create(&db, patch).await?;
    json_response(purchase)
}

async fn purchase_update(
    db: Arc<Db>,
    id: i64,
    patch: PurchasePatch,
) -> Result<Json, warp::Rejection> {
    let purchase = PurchaseMac::update(&db, id, patch).await?;
    json_response(purchase)
}

async fn purchase_delete(db: Arc<Db>, id: i64) -> Result<Json, warp::Rejection> {
    let purchase = PurchaseMac::delete(&db, id).await?;
    json_response(purchase)
}

fn json_response<D: Serialize + std::fmt::Debug>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
#[path = "../tests/web_purchase.rs"]
mod tests;
