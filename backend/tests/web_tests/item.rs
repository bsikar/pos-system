use crate::model::{init_db, item::Item};
use crate::web::item::item_rest_filters;
use actix_web::test::{self, TestRequest};
use actix_web::{web::Data, App};
use serde_json::json;

#[actix_rt::test]
async fn web_actix_item_list() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(item_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/items")
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let body: Vec<Item> = test::read_body_json(resp).await;

    // check body
    assert_eq!(body.len(), 3, "items count");
    let json = json!([
        {"name": "single glazed donut", "price": 120, "tax": 1.0},
        {"name": "half dozen glazed donuts", "price": 625, "tax": 1.0},
        {"name": "dozen glazed donuts", "price": 1099, "tax": 1.0}
    ]);

    // single glazed donut
    assert_eq!(body[0].name, json[0]["name"]);
    assert_eq!(body[0].price, json[0]["price"]);

    // half dozen donut
    assert_eq!(body[1].name, json[1]["name"]);
    assert_eq!(body[1].price, json[1]["price"]);

    // dozen donut
    assert_eq!(body[2].name, json[2]["name"]);
    assert_eq!(body[2].price, json[2]["price"]);
}

#[actix_rt::test]
async fn web_actix_item_get_ok() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(item_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/items/single%20glazed%20donut")
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let body: Item = test::read_body_json(resp).await;

    // check body
    assert_eq!(body.name, "single glazed donut");
    assert_eq!(body.price, 120);
}

#[actix_rt::test]
async fn web_actix_item_wrong_name() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(item_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/items/wrong%20name")
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_server_error()); // TODO make client error
}

#[actix_rt::test]
async fn web_actix_item_create_ok() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(item_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "single donut hole", "price": 30, "tax": 1.0}))
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let body: Item = test::read_body_json(resp).await;

    // check body
    assert_eq!(body.name, "single donut hole");
    assert_eq!(body.price, 30);
}

#[actix_rt::test]
async fn web_actix_item_create_duplicate() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(item_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "single glazed donut", "price": 120, "tax": 1.0}))
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_server_error()); // TODO make client error
}
