use crate::app::model::purchase::Purchase;
use crate::app::model::Error as ModelError;
use crate::app::App;
use chrono::Local;
use serde_json::json;

#[actix_rt::test]
async fn model_purchase_create_err_1() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!({});

    let result = Purchase::create(&conn, items);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_purchase_create_err_2() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!([]);

    let result = Purchase::create(&conn, items);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_purchase_create_err_3() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!(null);

    let result = Purchase::create(&conn, items);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_purchase_create_not_in_database() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!([{"name": "some random item", "price": 120, "quantity" : 1, "tax": 0.0, "type": "food"}]);

    let result = Purchase::create(&conn, items);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::ItemNotFound(name)) => assert_eq!(name, "some random item"),
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_purchase_create_ok_1() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!([{"name": "single glazed donut", "price": 120, "quantity" : 1, "tax": 0.0, "type": "food"}]);
    let time_before = Local::now().naive_local();

    let purchase_created = Purchase::create(&conn, items.clone()).unwrap();

    assert_eq!(purchase_created.items_to_json(), items);
    assert_eq!(purchase_created.total, 120);
    assert!(purchase_created.id >= 1, "id should be >= 1");
    assert!(purchase_created.ctime_to_ndt() >= time_before);
    assert!(purchase_created.ctime_to_ndt() <= Local::now().naive_local());
}

#[actix_rt::test]
async fn model_purchase_create_ok_2() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!([{"name": "single glazed donut", "price": 120, "quantity": 1, "tax": 0.0, "type": "food"}, {"name": "half dozen glazed donuts", "price": 625, "quantity" : 1, "tax": 0.0, "type": "food"}]);

    let time_before = Local::now().naive_local();
    let purchase_created = Purchase::create(&conn, items.clone()).unwrap();

    assert_eq!(purchase_created.items_to_json(), items);
    assert_eq!(purchase_created.total, 745);
    assert!(purchase_created.id >= 1, "id should be >= 1");
    assert!(purchase_created.ctime_to_ndt() >= time_before);
    assert!(purchase_created.ctime_to_ndt() <= Local::now().naive_local());
}

#[actix_rt::test]
async fn model_purchase_get_ok_1() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let purchase = Purchase::get_by_id(&conn, 1).unwrap();

    println!("{:#?}", purchase);

    // check
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity" : 1}]);
    assert_eq!(purchase.id, 1);
    assert_eq!(purchase.items_to_json(), json);
    assert_eq!(purchase.total, 120);
}

#[actix_rt::test]
async fn model_purchase_get_ok_2() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let purchase = Purchase::get_by_id(&conn, 3).unwrap();

    // check
    let json = json!([
        {"name": "half dozen glazed donuts", "price": 625, "quantity" : 1},
        {"name": "dozen glazed donuts", "price": 1099, "quantity" : 2},
    ]);

    assert_eq!(purchase.id, 3);
    assert_eq!(purchase.items_to_json(), json);
    assert_eq!(purchase.total, 2823);
}

#[actix_rt::test]
async fn model_purchase_get_wrong_id() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let result = Purchase::get_by_id(&conn, -1);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::PurchaseNotFound(id)) => assert_eq!(id, -1),
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_purchase_list() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let purchases = Purchase::list(&conn).unwrap();

    assert_eq!(purchases.len(), 3, "number of seed purchases");

    let json = json!([
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 1}, {"name": "dozen glazed donuts", "price": 1099, "quantity": 2}],
        [{"name": "half dozen glazed donuts", "price": 625, "quantity": 2}],
        [{"name": "single glazed donut", "price": 120, "quantity": 1}],
    ]);

    assert_eq!(purchases[0].id, 1);
    assert_eq!(purchases[0].items_to_json(), json[2]);
    assert_eq!(purchases[0].total, 120);

    assert_eq!(purchases[1].id, 2);
    assert_eq!(purchases[1].items_to_json(), json[1]);
    assert_eq!(purchases[1].total, 1250);

    assert_eq!(purchases[2].id, 3);
    assert_eq!(purchases[2].items_to_json(), json[0]);
    assert_eq!(purchases[2].total, 2823);

    assert!(purchases[0].ctime_to_ndt().timestamp() >= purchases[1].ctime_to_ndt().timestamp());
    assert!(purchases[1].ctime_to_ndt().timestamp() >= purchases[2].ctime_to_ndt().timestamp());
}

#[actix_rt::test]
async fn model_purchase_delete() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let purchase = Purchase::delete(&conn, 1).unwrap();

    let json = json!([{"name": "single glazed donut", "price": 120, "quantity" : 1}]);
    assert_eq!(purchase.id, 1);
    assert_eq!(purchase.items_to_json(), json);
    assert_eq!(purchase.total, 120);

    // check - list
    let list = Purchase::list(&conn).unwrap();
    assert_eq!(list.len(), 2);
}

#[actix_rt::test]
async fn model_purchase_delete_wrong_id() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let purchase = Purchase::delete(&conn, -1);

    match purchase {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::PurchaseNotFound(id)) => assert_eq!(id, -1),
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_purchase_update_empty_items() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!([]);

    let purchase = Purchase::update(&conn, 100, items);

    match purchase {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_purchase_update() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = json!([
        {"name": "single glazed donut", "price": 120, "quantity" : 1, "tax": 0.0, "type": "food"},
        {"name": "half dozen glazed donuts", "price": 625, "quantity" : 2, "tax": 0.0, "type": "food"},
    ]);

    let purchase = Purchase::update(&conn, 1, items.clone()).unwrap();

    assert_eq!(purchase.id, 1);
    assert_eq!(purchase.items_to_json(), items);
    assert_eq!(purchase.total, 1370);
}

#[actix_rt::test]
async fn model_purchase_calculate_total_1() {
    let json = json!([{"name": "single donut hole", "price": 30, "quantity": 1, "tax": 0.0}]);

    assert_eq!(Purchase::calculate_total(&json), 30);
}

#[actix_rt::test]
async fn model_purchase_calculate_total_2() {
    let json = json!([{"name": "single donut hole", "price": 30, "quantity": 2, "tax": 0.0}, {"name": "dozen donut holes", "price": 200, "quantity": 1, "tax": 0.0}]);

    assert_eq!(Purchase::calculate_total(&json), 260);
}
