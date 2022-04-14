use crate::app::model::item::Item;
use crate::app::model::Error as ModelError;
use crate::app::App;

#[actix_rt::test]
async fn model_item_create_ok() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let item = Item {
        name: "single donut hole".to_string(),
        price: 30,
        tax: 1.0,
        type_: "food".to_string(),
    };

    let item_created = Item::create(&conn, item).unwrap();

    assert_eq!(item_created.name, "single donut hole");
    assert_eq!(item_created.price, 30);
    assert_eq!(item_created.tax, 1.0);
    assert_eq!(item_created.type_, "food");
}

#[actix_rt::test]
async fn model_item_create_duplicate() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let item = Item {
        name: "single glazed donut".to_string(),
        price: 220,
        tax: 1.0,
        type_: "food".to_string(),
    };

    let result = Item::create(&conn, item);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::ItemAlreadyExists(name)) => assert_eq!(name, "single glazed donut"),

        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_item_create_invalid_price() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let item = Item {
        name: "single donut hole".to_string(),
        price: -30,
        tax: 1.0,
        type_: "food".to_string(),
    };

    let result = Item::create(&conn, item);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::InvalidItemPrice(price)) => assert_eq!(price, -30),
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_item_create_invalid_tax() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let item = Item {
        name: "single donut hole".to_string(),
        price: 30,
        tax: -1.0,
        type_: "food".to_string(),
    };

    let result = Item::create(&conn, item);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::InvalidItemTax(tax)) => assert_eq!(tax, -1.0),

        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_item_create_empty_name() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let item = Item {
        name: "".to_string(),
        price: 30,
        tax: 1.0,
        type_: "food".to_string(),
    };

    let result = Item::create(&conn, item);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::EmptyItemName) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_item_get_wrong_name() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let result = Item::get_by_name(&conn, "wrong name".to_string());

    match result {
        Ok(_) => {}
        Err(ModelError::ItemNotFound(name)) => assert_eq!(name, "wrong name"),
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_item_list() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let items = Item::list(&conn).unwrap();

    assert_eq!(items.len(), 3, "number of seed items");

    assert_eq!(items[0].name, "single glazed donut");
    assert_eq!(items[0].price, 120);
    assert_eq!(items[0].tax, 1.0);
    assert_eq!(items[0].type_, "food");

    assert_eq!(items[1].name, "half dozen glazed donuts");
    assert_eq!(items[1].price, 625);
    assert_eq!(items[1].tax, 1.0);
    assert_eq!(items[1].type_, "food");

    assert_eq!(items[2].name, "dozen glazed donuts");
    assert_eq!(items[2].price, 1099);
    assert_eq!(items[2].tax, 1.0);
    assert_eq!(items[2].type_, "food");
}

#[actix_rt::test]
async fn model_item_delete() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let item = Item::delete(&conn, "single glazed donut".to_string()).unwrap();

    assert_eq!(item.name, "single glazed donut");
    assert_eq!(item.price, 120);

    let list = Item::list(&conn).unwrap();

    assert_eq!(list.len(), 2);

    assert_eq!(list[0].name, "half dozen glazed donuts");
    assert_eq!(list[0].price, 625);
    assert_eq!(list[0].tax, 1.0);
    assert_eq!(list[0].type_, "food");

    assert_eq!(list[1].name, "dozen glazed donuts");
    assert_eq!(list[1].price, 1099);
    assert_eq!(list[1].tax, 1.0);
    assert_eq!(list[1].type_, "food");
}

#[actix_rt::test]
async fn model_item_delete_wrong_name() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let result = Item::delete(&conn, "wrong name".to_string());

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::ItemNotFound(name)) => assert_eq!(name, "wrong name"),
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}

#[actix_rt::test]
async fn model_item_update() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let name = "single glazed donut".to_string();
    let item = Item {
        name: name.clone(),
        price: 999,
        tax: 1.0,
        type_: "food".to_string(),
    };

    let item_updated = Item::update(&conn, name, item).unwrap();

    assert_eq!(item_updated.name, "single glazed donut");
    assert_eq!(item_updated.price, 999);
    assert_eq!(item_updated.tax, 1.0);
    assert_eq!(item_updated.type_, "food");
}
