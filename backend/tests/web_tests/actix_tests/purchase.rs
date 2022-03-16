/*
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
    assert_eq!(purchases[0].id, 102);
    assert_eq!(purchases[0].items, json[0]);
    assert_eq!(purchases[0].total, 2823);

    // purchase 101
    assert_eq!(purchases[1].id, 101);
    assert_eq!(purchases[1].items, json[1]);
    assert_eq!(purchases[1].total, 1250);

    // purchase 100
    assert_eq!(purchases[2].id, 100);
    assert_eq!(purchases[2].items, json[2]);
    assert_eq!(purchases[2].total, 120);
}
*/
