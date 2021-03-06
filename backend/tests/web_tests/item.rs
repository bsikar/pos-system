use crate::app::web::item::{item_rest_filters, Item};
use crate::app::App as ModelApp;
use actix_web::test::{self, TestRequest};
use actix_web::{web::Data, App};
use serde_json::{json, Value as JsonValue};

#[actix_rt::test]
async fn web_item_list_all() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items")
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: Vec<JsonValue> = test::read_body_json(resp).await;
    let body = body.into_iter().map(Item::from_json).collect::<Vec<_>>();

    assert_eq!(body.len(), 9, "items count");
    let json = json!([
        {"name": "single glazed donut", "price": 120, "tax": 1.0, "type": "food"},
        {"name": "half dozen glazed donuts", "price": 625, "tax": 1.0, "type": "food"},
        {"name": "dozen glazed donuts", "price": 1099, "tax": 1.0, "type": "food"},
        {"name": "small chocolate milk", "price": 169, "tax": 1.0, "type": "drink"},
        {"name": "large chocolate milk", "price": 249, "tax": 1.0, "type": "drink"},
        {"name": "energy drink", "price": 300, "tax": 1.25, "type": "drink"},
        {"name": "painting one", "price": 5000, "tax": 1.0, "type": "other"},
        {"name": "painting two", "price": 10000, "tax": 1.0, "type": "other"},
        {"name": "painting three", "price": 7500, "tax": 1.0, "type": "other"},
    ]);

    for i in 0..body.len() {
        assert_eq!(body[i].name, json[i]["name"].as_str().unwrap());
        assert_eq!(body[i].price, json[i]["price"].as_i64().unwrap() as i32);
        assert_eq!(body[i].tax, json[i]["tax"].as_f64().unwrap() as f32);
        assert_eq!(body[i].type_, json[i]["type"].as_str().unwrap());
    }
}

#[actix_rt::test]
async fn web_item_list_food() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items/food")
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: Vec<JsonValue> = test::read_body_json(resp).await;
    let body = body.into_iter().map(Item::from_json).collect::<Vec<_>>();

    assert_eq!(body.len(), 3, "items count");
    let json = json!([
        {"name": "single glazed donut", "price": 120, "tax": 1.0, "type": "food"},
        {"name": "half dozen glazed donuts", "price": 625, "tax": 1.0, "type": "food"},
        {"name": "dozen glazed donuts", "price": 1099, "tax": 1.0, "type": "food"},
    ]);

    for i in 0..body.len() {
        assert_eq!(body[i].name, json[i]["name"].as_str().unwrap());
        assert_eq!(body[i].price, json[i]["price"].as_i64().unwrap() as i32);
        assert_eq!(body[i].tax, json[i]["tax"].as_f64().unwrap() as f32);
        assert_eq!(body[i].type_, json[i]["type"].as_str().unwrap());
    }
}

#[actix_rt::test]
async fn web_item_list_drink() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items/drinks")
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: Vec<JsonValue> = test::read_body_json(resp).await;
    let body = body.into_iter().map(Item::from_json).collect::<Vec<_>>();

    assert_eq!(body.len(), 3, "items count");
    let json = json!([
        {"name": "small chocolate milk", "price": 169, "tax": 1.0, "type": "drink"},
        {"name": "large chocolate milk", "price": 249, "tax": 1.0, "type": "drink"},
        {"name": "energy drink", "price": 300, "tax": 1.25, "type": "drink"},
    ]);

    for i in 0..body.len() {
        assert_eq!(body[i].name, json[i]["name"].as_str().unwrap());
        assert_eq!(body[i].price, json[i]["price"].as_i64().unwrap() as i32);
        assert_eq!(body[i].tax, json[i]["tax"].as_f64().unwrap() as f32);
        assert_eq!(body[i].type_, json[i]["type"].as_str().unwrap());
    }
}

#[actix_rt::test]
async fn web_item_list_other() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items/other")
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: Vec<JsonValue> = test::read_body_json(resp).await;
    let body = body.into_iter().map(Item::from_json).collect::<Vec<_>>();

    assert_eq!(body.len(), 3, "items count");
    let json = json!([
        {"name": "painting one", "price": 5000, "tax": 1.0, "type": "other"},
        {"name": "painting two", "price": 10000, "tax": 1.0, "type": "other"},
        {"name": "painting three", "price": 7500, "tax": 1.0, "type": "other"},
    ]);

    for i in 0..body.len() {
        assert_eq!(body[i].name, json[i]["name"].as_str().unwrap());
        assert_eq!(body[i].price, json[i]["price"].as_i64().unwrap() as i32);
        assert_eq!(body[i].tax, json[i]["tax"].as_f64().unwrap() as f32);
        assert_eq!(body[i].type_, json[i]["type"].as_str().unwrap());
    }
}

#[actix_rt::test]
async fn web_item_get_ok() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items/name/single%20glazed%20donut")
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: JsonValue = test::read_body_json(resp).await;
    let body = Item::from_json(body);

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

    let app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/items/name/wrong%20name")
        .send_request(&app)
        .await;

    assert!(resp.status().is_server_error()); // TODO make client error
}

#[actix_rt::test]
async fn web_item_create_ok() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "single donut hole", "price": 30, "tax": 1.0, "type": "food"}))
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: JsonValue = test::read_body_json(resp).await;
    let body = Item::from_json(body);

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

    let app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "single glazed donut", "price": 120, "tax": 1.0, "type": "food"}))
        .send_request(&app)
        .await;

    assert!(resp.status().is_server_error()); // TODO make client error
}

#[actix_rt::test]
async fn web_item_create_food() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "long john donut", "price": 125, "tax": 1.0, "type": "food"}))
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: JsonValue = test::read_body_json(resp).await;
    let body = Item::from_json(body);

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

    let app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "small coffee", "price": 200, "tax": 1.25, "type": "drink"}))
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: JsonValue = test::read_body_json(resp).await;
    let body = Item::from_json(body);

    assert_eq!(body.name, "small coffee");
    assert_eq!(body.price, 200);
    assert_eq!(body.tax, 1.25);
    assert_eq!(body.type_, "drink");
}

#[actix_rt::test]
async fn web_item_create_other() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(item_rest_filters);

    let app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(&json!({"name": "amazing art", "price": 999, "tax": 100.0, "type": "other"}))
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    let body: JsonValue = test::read_body_json(resp).await;
    let body = Item::from_json(body);

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

    let app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/items")
        .set_json(
            &json!({"name": "invalid type", "price": 999, "tax": 100.0, "type": "something-random"}),
        )
        .send_request(&app)
        .await;

    assert!(resp.status().is_server_error());
}
