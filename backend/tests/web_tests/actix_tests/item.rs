use crate::model::init_db;
use crate::web::actix::item::item_rest_filters;
use actix_web::{test::TestRequest, web::Data, App};
use std::sync::Arc;

#[actix_rt::test]
async fn web_actix_item_list() {
    // fixture
    let db = init_db().await;
    let db = Arc::new(db);
    let app = App::new()
        .app_data(Data::new(db.clone()))
        .configure(item_rest_filters);
    let mut app = actix_web::test::init_service(app).await;

    // action
    let resp = TestRequest::get()
        .uri("/api/items")
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());
}
