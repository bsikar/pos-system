use crate::app::web::purchase::{purchase_rest_filters, Purchase};
use crate::app::App as ModelApp;
use actix_web::test::{self, TestRequest};
use actix_web::{web::Data, App};
use serde_json::json;

#[actix_rt::test]
async fn web_purchase_list() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/purchases")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Vec<Purchase> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3, "purchases count");
    let json = json!([
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}],
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 2}],
        [{"name": "single glazed donut", "price": 120, "quantity": 1}],
    ]);

    assert_eq!(body[0].id, 100);
    assert_eq!(body[0].items, json[2]);
    assert_eq!(body[0].total, 120);
    assert!(body[0].ctime.timestamp() > 0);
    assert!(body[0].ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    assert_eq!(body[1].id, 101);
    assert_eq!(body[1].items, json[1]);
    assert_eq!(body[1].total, 1250);
    assert!(body[1].ctime.timestamp() > 0);
    assert!(body[1].ctime.timestamp() <= chrono::offset::Utc::now().timestamp());

    assert_eq!(body[2].id, 102);
    assert_eq!(body[2].items, json[0]);
    assert_eq!(body[2].total, 2823);
    assert!(body[2].ctime.timestamp() > 0);
    assert!(body[2].ctime.timestamp() <= chrono::offset::Utc::now().timestamp());
}

#[actix_rt::test]
async fn web_purchase_get_ok_1() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/purchases/101")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Purchase = test::read_body_json(resp).await;

    assert_eq!(body.id, 101);
    assert_eq!(
        body.items,
        json!([{"name": "half dozen glazed donuts", "price": 625, "quantity": 2}])
    );
    assert_eq!(body.total, 1250);
    assert!(body.ctime.timestamp() > 0);
    assert!(body.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());
}

#[actix_rt::test]
async fn web_purchase_get_ok_2() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/purchases/102")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let body: Purchase = test::read_body_json(resp).await;

    assert_eq!(body.id, 102);
    assert_eq!(
        body.items,
        json!([{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}])
    );
    assert_eq!(body.total, 2823);
    assert!(body.ctime.timestamp() > 0);
    assert!(body.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());
}

#[actix_rt::test]
async fn web_purchase_get_wrong_id() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::get()
        .uri("/api/purchases/999")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_server_error());
}

#[actix_rt::test]
async fn web_purchase_create_wrong_name() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&json!({
            "items": [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1, "tax": 1.0}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2, "tax": 1.0}],
            "total": 2823,
        }))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn web_purchase_create_wrong_price() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&json!({
            "items": [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1, "tax": 1.0}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2, "tax": 1.0}],
            "total": 2823,
        }))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn web_purchase_create_ok_1() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let json =
        json!([{"name": "dozen glazed donuts", "price": 1099i64, "quantity": 1i64, "tax": 1.0}]);
    let body = json!({"items": json, "total": Purchase::calculate_total(&json)});

    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&body)
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let purchase: Purchase = test::read_body_json(resp).await;

    assert!(purchase.id >= 1000, "purchase.id should be >= to 1000");
    assert_eq!(purchase.items, body["items"]);
    assert_eq!(purchase.total, 1099);
    assert!(purchase.ctime.timestamp() > 0);
    assert!(purchase.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());
}

#[actix_rt::test]
async fn web_purchase_create_ok_2() {
    let conn = ModelApp::new().unwrap().database.establish_db_conn();

    let app = App::new()
        .app_data(Data::new(conn))
        .configure(purchase_rest_filters);

    let mut app = test::init_service(app).await;

    let json = json!([{"name": "single glazed donut", "price": 120, "quantity": 1, "tax": 1.0}, {"name": "half dozen glazed donuts", "price": 625, "quantity": 2, "tax": 1.0}]);
    let body = json!({"items": json, "total": Purchase::calculate_total(&json)});

    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&body)
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());

    let purchase: Purchase = test::read_body_json(resp).await;

    assert!(purchase.id >= 1000, "purchase.id should be >= to 1000");
    assert_eq!(purchase.items, body["items"]);
    assert_eq!(purchase.total, 1370);
    assert!(purchase.ctime.timestamp() > 0);
    assert!(purchase.ctime.timestamp() <= chrono::offset::Utc::now().timestamp());
}