use crate::model::{init_db, Purchase};
use crate::web::purchase_rest_filters;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
use std::str::from_utf8;
use std::sync::Arc;
use warp::hyper::body::Bytes;
use warp::hyper::Response;

#[tokio::test]
async fn web_purchase_list() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters("api", db.clone());

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
    assert_eq!(purchases.len(), 2, "number of seed purchases");
    let json = json!([
        { "items": [{"name": "test 2", "price": 200, "quantity" : 2}] },
        { "items": [{"name": "test 1", "price": 100, "quantity" : 1}] },
    ]);
    // purchase 101
    assert_eq!(purchases[0].id, 101);
    assert_eq!(purchases[0].items, json[0]);
    assert_eq!(purchases[0].total, 400);

    // purchase 100
    assert_eq!(purchases[1].id, 100);
    assert_eq!(purchases[1].items, json[1]);
    assert_eq!(purchases[1].total, 100);

    Ok(())
}

#[tokio::test]
async fn web_purchase_get_ok() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters("api", db);

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
    let json = json!({ "items": [{"name": "test 1", "price": 100, "quantity" : 1}] });

    // check data
    assert_eq!(purchase.id, 100);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 100);

    Ok(())
}

#[tokio::test]
async fn web_purchase_create_ok() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let purchase_apis = purchase_rest_filters("api", db);

    // new purchase fixture
    let body = json!({
        "items": [
            { "name": "test 1", "price": 100, "quantity": 1 },
            { "name": "test 2", "price": 200, "quantity": 2 },
            { "name": "test 3", "price": 300, "quantity": 3 },
            { "name": "test 4", "price": 400, "quantity": 4 },
            { "name": "test 5", "price": 500, "quantity": 5 },
        ]
    });

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
    assert_eq!(purchase.items, body);
    assert_eq!(purchase.total, 5500);

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
