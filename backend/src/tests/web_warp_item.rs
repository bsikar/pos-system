use crate::model::{init_db, item::Item};
use crate::web::warp::item_rest_filters;
use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
use serde_json::{from_str, from_value, Value};
use std::str::from_utf8;
use std::sync::Arc;
use warp::hyper::body::Bytes;
use warp::hyper::Response;

#[tokio::test]
async fn web_item_list() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let item_apis = item_rest_filters(db);

    // action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/items")
        .reply(&item_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let items: Vec<Item> = extract_body_data(resp)?;

    // check data
    assert_eq!(items.len(), 3, "items count");
    let json = json!([
        {"name": "single glazed donut", "price": 120},
        {"name": "half dozen glazed donuts", "price": 625},
        {"name": "dozen glazed donuts", "price": 1099}
    ]);

    // single glazed donut
    assert_eq!(items[0].name, json[0]["name"]);
    assert_eq!(items[0].price, json[0]["price"]);

    // half dozen donut
    assert_eq!(items[1].name, json[1]["name"]);
    assert_eq!(items[1].price, json[1]["price"]);

    // dozen donut
    assert_eq!(items[2].name, json[2]["name"]);
    assert_eq!(items[2].price, json[2]["price"]);

    Ok(())
}

#[tokio::test]
async fn web_item_get_ok() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let item_apis = item_rest_filters(db);

    // action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/items/single%20glazed%20donut")
        .reply(&item_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let item: Item = extract_body_data(resp)?;
    let json = json!({"name": "single glazed donut", "price": 120});

    // check data
    assert_eq!(item.name, json["name"]);
    assert_eq!(item.price, json["price"]);

    Ok(())
}

#[tokio::test]
async fn web_item_wrong_name() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let item_apis = item_rest_filters(db);

    // action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/items/wrong%20name")
        .reply(&item_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 500, "http status");

    Ok(())
}

#[tokio::test]
async fn web_item_create_ok() -> Result<()> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let item_apis = item_rest_filters(db);

    // new item
    let json = json!({"name": "single donut hole", "price": 30});

    // action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/items")
        .json(&json)
        .reply(&item_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 200, "http status");

    // extract response .data
    let item: Item = extract_body_data(resp)?;

    // check data
    assert_eq!(item.name, json["name"]);
    assert_eq!(item.price, json["price"]);

    Ok(())
}

#[tokio::test]
async fn web_item_create_duplicate() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let item_apis = item_rest_filters(db);

    // new item
    let json = json!({"name": "single glazed donut", "price": 120});

    // action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/items")
        .json(&json)
        .reply(&item_apis)
        .await;

    // check status
    assert_eq!(resp.status(), 500, "http status");

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
