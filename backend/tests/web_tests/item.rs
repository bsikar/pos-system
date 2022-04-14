use crate::app::web::item::{item_rest_filters, Item};
use crate::app::App as ModelApp;
use actix_web::test::{self, TestRequest};
use actix_web::{web::Data, App};
use serde_json::json;

#[actix_rt::test]
async fn web_item_list() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Vec<Item> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3, "items count");
    let json = json!([
        {"name": "single glazed donut", "price": 120, "tax": 1.0, "type": "food"},
        {"name": "half dozen glazed donuts", "price": 625, "tax": 1.0, "type": "food"},
        {"name": "dozen glazed donuts", "price": 1099, "tax": 1.0, "type": "food"},
    ]);

    assert_eq!(body[0].name, json[0]["name"]);
    assert_eq!(body[0].price, json[0]["price"]);
    assert_eq!(body[0].tax, json[0]["tax"]);
    assert_eq!(body[0].type_, json[0]["type"]);

    assert_eq!(body[1].name, json[1]["name"]);
    assert_eq!(body[1].price, json[1]["price"]);
    assert_eq!(body[1].tax, json[1]["tax"]);
    assert_eq!(body[1].type_, json[1]["type"]);

    assert_eq!(body[2].name, json[2]["name"]);
    assert_eq!(body[2].price, json[2]["price"]);
    assert_eq!(body[2].tax, json[2]["tax"]);
    assert_eq!(body[2].type_, json[2]["type"]);
}

#[actix_rt::test]
async fn web_item_get_ok() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items/single%20glazed%20donut")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Item = test::read_body_json(resp).await;

    assert_eq!(body.name, "single glazed donut");
    assert_eq!(body.price, 120);
    assert_eq!(body.tax, 1.0);
    assert_eq!(body.type_, "food");
}

#[actix_rt::test]
async fn web_item_wrong_name() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items/wrong%20name")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_server_error()); // TODO make client error
}

#[actix_rt::test]
async fn web_item_create_ok() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "single donut hole", "price": 30, "tax": 1.0, "type_": "food"}))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Item = test::read_body_json(resp).await;

    assert_eq!(body.name, "single donut hole");
    assert_eq!(body.price, 30);
    assert_eq!(body.tax, 1.0);
    assert_eq!(body.type_, "food");
}

#[actix_rt::test]
async fn web_item_create_duplicate() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(
            &json!({"name": "single glazed donut", "price": 120, "tax": 1.0, "type_": "food"}),
        )
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_server_error()); // TODO make client error
}

#[actix_rt::test]
async fn web_item_create_food() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "long john donut", "price": 125, "tax": 1.0, "type_": "food"}))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Item = test::read_body_json(resp).await;

    assert_eq!(body.name, "long john donut");
    assert_eq!(body.price, 125);
    assert_eq!(body.tax, 1.0);
    assert_eq!(body.type_, "food");
}

#[actix_rt::test]
async fn web_item_create_drink() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "energy drink", "price": 300, "tax": 1.0, "type_": "drink"}))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Item = test::read_body_json(resp).await;

    assert_eq!(body.name, "energy drink");
    assert_eq!(body.price, 300);
    assert_eq!(body.tax, 1.0);
    assert_eq!(body.type_, "drink");
}

#[actix_rt::test]
async fn web_item_create_other() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "amazing art", "price": 999, "tax": 100.0, "type_": "other"}))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Item = test::read_body_json(resp).await;

    assert_eq!(body.name, "amazing art");
    assert_eq!(body.price, 999);
    assert_eq!(body.tax, 100.0);
    assert_eq!(body.type_, "other");
}

#[actix_rt::test]
async fn web_item_create_invalid_type() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(
            &json!({"name": "invalid type", "price": 999, "tax": 100.0, "type_": "something-random"}),
        )
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_server_error());
}
