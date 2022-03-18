use crate::app::model::{init_db, purchase::calculate_total, Purchase};
use crate::app::web::warp::purchase_rest_filters;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
use std::{str::from_utf8, sync::Arc};
use warp::hyper::{body::Bytes, Response};

#[tokio::test]
async fn web_warp_purchase_list() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/purchases")
        .reply(&purchase_apis)
        .await;

    // chcek status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let purchases: Vec<Purchase> = extract_body_data(resp)?;

    // check data
    let json = json!([
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}],
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 2}],
        [{"name": "single glazed donut", "price": 120, "quantity": 1}],
    ]);
    // purchase 102
    assert_eq!(purchases[0].id, 102);
    assert_eq!(purchases[0].items, json[0]);
    assert_eq!(purchases[0].total, 2823);
    assert!(purchases[0].ctime.timestamp() > 0);
    assert!(purchases[0].ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    // purchase 101
    assert_eq!(purchases[1].id, 101);
    assert_eq!(purchases[1].items, json[1]);
    assert_eq!(purchases[1].total, 1250);
    assert!(purchases[1].ctime.timestamp() > 0);
    assert!(purchases[1].ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    // purchase 100
    assert_eq!(purchases[2].id, 100);
    assert_eq!(purchases[2].items, json[2]);
    assert_eq!(purchases[2].total, 120);
    assert!(purchases[2].ctime.timestamp() > 0);
    assert!(purchases[2].ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    Ok(())
}

#[tokio::test]
async fn web_warp_purchase_get_ok_1() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/purchases/100")
        .reply(&purchase_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let purchase: Purchase = extract_body_data(resp)?;
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity": 1}]);

    // check data
    assert_eq!(purchase.id, 100);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 120);
    assert!(purchase.ctime.timestamp() > 0);
    assert!(purchase.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    Ok(())
}

#[tokio::test]
async fn web_warp_purchase_get_ok_2() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/purchases/102")
        .reply(&purchase_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let purchase: Purchase = extract_body_data(resp)?;
    let json = json!([{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}]);

    // check data
    assert_eq!(purchase.id, 102);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 2823);
    assert!(purchase.ctime.timestamp() > 0);
    assert!(purchase.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    Ok(())
}

#[tokio::test]
async fn web_warp_purchase_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/purchases/10")
        .reply(&purchase_apis)
        .await;

    assert_eq!(resp.status(), 500, "http status");

    Ok(())
}

#[tokio::test]
async fn web_warp_purchase_create_wrong_name() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // new purchase fixture
    let json = json!([{"name": "wrong name", "price": 1099, "quantity": 1}]);
    let body = json!({"items": json, "total": calculate_total(&json)});

    // action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/purchases")
        .json(&body)
        .reply(&purchase_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 500, "http status");

    Ok(())
}

#[tokio::test]
async fn web_warp_purchase_create_wrong_price() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // new purchase fixture
    let json = json!([{"name": "single glazed donut", "price": 9999, "quantity": 1}]);
    let body = json!({"items": json, "total": calculate_total(&json)});

    // action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/purchases")
        .json(&body)
        .reply(&purchase_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 500, "http status");

    Ok(())
}

#[tokio::test]
async fn web_warp_purchase_create_ok_1() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // new purchase fixture
    let json = json!([{"name": "dozen glazed donuts", "price": 1099, "quantity": 1}]);
    let body = json!({"items": json, "total": calculate_total(&json)});

    // action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/purchases")
        .json(&body)
        .reply(&purchase_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let purchase: Purchase = extract_body_data(resp)?;

    // check data
    assert!(purchase.id >= 1000, "purchase.id should be >= to 1000");
    assert_eq!(purchase.items, body["items"]);
    assert_eq!(purchase.total, 1099);
    assert!(purchase.ctime.timestamp() > 0);
    assert!(purchase.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    Ok(())
}

#[tokio::test]
async fn web_warp_purchase_create_ok_2() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters(db);

    // new purchase
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity": 1}, {"name": "half dozen glazed donuts", "price": 625, "quantity": 2}]);
    let body = json!({"items": json, "total": calculate_total(&json)});

    // action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/purchases")
        .json(&body)
        .reply(&purchase_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let purchase: Purchase = extract_body_data(resp)?;

    // check data
    assert!(purchase.id >= 1000, "purchase.id should be >= to 1000");
    assert_eq!(purchase.items, body["items"]);
    assert_eq!(purchase.total, 1370);
    assert!(purchase.ctime.timestamp() > 0);
    assert!(purchase.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    Ok(())
}

fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
    for<'de> D: Deserialize<'de>,
{
    // parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value = from_str(body)
        .with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;

    // extract the data
    let data = body["data"].take();

    // deserialize the data to D
    let data: D = from_value(data)?;

    Ok(data)
}
