use crate::model::purchase::calculate_total;
use crate::model::{init_db, purchase::Purchase};
use crate::web::actix::purchase::purchase_rest_filters;
use actix_web::test::{self, TestRequest};
use actix_web::{web::Data, App};
use serde_json::json;

#[actix_rt::test]
async fn web_actix_purchase_list() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/purchases")
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let body: Vec<Purchase> = test::read_body_json(resp).await;

    // check body
    assert_eq!(body.len(), 3, "purchases count");
    let json = json!([
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}],
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 2}],
        [{"name": "single glazed donut", "price": 120, "quantity": 1}],
    ]);

    // purchase 102
    assert_eq!(body[0].id, 102);
    assert_eq!(body[0].items, json[0]);
    assert_eq!(body[0].total, 2823);

    // purchase 101
    assert_eq!(body[1].id, 101);
    assert_eq!(body[1].items, json[1]);
    assert_eq!(body[1].total, 1250);

    // purchase 100
    assert_eq!(body[2].id, 100);
    assert_eq!(body[2].items, json[2]);
    assert_eq!(body[2].total, 120);
}

#[actix_rt::test]
async fn web_actix_purchase_get_ok_1() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/purchases/101")
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let body: Purchase = test::read_body_json(resp).await;

    // check body
    assert_eq!(body.id, 101);
    assert_eq!(
        body.items,
        json!([{"name": "half dozen glazed donuts", "price": 625, "quantity": 2}])
    );
    assert_eq!(body.total, 1250);
}

#[actix_rt::test]
async fn web_actix_purchase_get_ok_2() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/purchases/102")
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let body: Purchase = test::read_body_json(resp).await;

    // check body
    assert_eq!(body.id, 102);
    assert_eq!(
        body.items,
        json!([{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}])
    );
    assert_eq!(body.total, 2823);
}

#[actix_rt::test]
async fn web_actix_purchase_get_wrong_id() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/purchases/999")
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_server_error());
}

#[actix_rt::test]
async fn web_actix_purchase_create_wrong_name() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&json!({
            "items": [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}],
            "total": 2823,
        }))
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn web_actix_purchase_create_wrong_price() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    // action
    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&json!({
            "items": [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}],
            "total": 2823,
        }))
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn web_actix_purchase_create_ok_1() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    let json = json!([{"name": "dozen glazed donuts", "price": 1099i64, "quantity": 1i64}]);
    let body = json!({"items": json, "total": calculate_total(&json)});

    // action
    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&body)
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let purchase: Purchase = test::read_body_json(resp).await;

    // check body
    assert!(purchase.id >= 1000, "purchase.id should be >= to 1000");
    assert_eq!(purchase.items, body["items"]);
    assert_eq!(purchase.total, 1099);
}

#[actix_rt::test]
async fn web_actix_purchase_create_ok_2() {
    // fixture
    let db = init_db().await.unwrap();
    let app = App::new()
        .app_data(Data::new(db))
        .configure(purchase_rest_filters);
    let mut app = test::init_service(app).await;

    // new purchase
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity": 1}, {"name": "half dozen glazed donuts", "price": 625, "quantity": 2}]);
    let body = json!({"items": json, "total": calculate_total(&json)});

    // action
    let resp = TestRequest::post()
        .uri("/api/purchases")
        .set_json(&body)
        .send_request(&mut app)
        .await;

    // check status
    assert!(resp.status().is_success());

    // extract body
    let purchase: Purchase = test::read_body_json(resp).await;

    // check body
    assert!(purchase.id >= 1000, "purchase.id should be >= to 1000");
    assert_eq!(purchase.items, body["items"]);
    assert_eq!(purchase.total, 1370);
}
