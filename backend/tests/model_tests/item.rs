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
async fn model_item_is_food() {
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

    assert!(item_created.is_food());
}

#[actix_rt::test]
async fn model_item_is_drink() {
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
        type_: "drink".to_string(),
    };

    let item_created = Item::create(&conn, item).unwrap();

    assert!(item_created.is_drink());
}

#[actix_rt::test]
async fn model_item_is_other() {
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
        type_: "other".to_string(),
    };

    let item_created = Item::create(&conn, item).unwrap();

    assert!(item_created.is_other());
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

    assert_eq!(items.len(), 9, "number of seed items");

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

    assert_eq!(items[3].name, "small chocolate milk");
    assert_eq!(items[3].price, 169);
    assert_eq!(items[3].tax, 1.0);
    assert_eq!(items[3].type_, "drink");

    assert_eq!(items[4].name, "large chocolate milk");
    assert_eq!(items[4].price, 249);
    assert_eq!(items[4].tax, 1.0);
    assert_eq!(items[4].type_, "drink");

    assert_eq!(items[5].name, "energy drink");
    assert_eq!(items[5].price, 300);
    assert_eq!(items[5].tax, 1.25);
    assert_eq!(items[5].type_, "drink");

    assert_eq!(items[6].name, "painting one");
    assert_eq!(items[6].price, 5000);
    assert_eq!(items[6].tax, 1.0);
    assert_eq!(items[6].type_, "other");

    assert_eq!(items[7].name, "painting two");
    assert_eq!(items[7].price, 10000);
    assert_eq!(items[7].tax, 1.0);
    assert_eq!(items[7].type_, "other");

    assert_eq!(items[8].name, "painting three");
    assert_eq!(items[8].price, 7500);
    assert_eq!(items[8].tax, 1.0);
    assert_eq!(items[8].type_, "other");
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

    assert_eq!(list.len(), 8);

    assert_eq!(list[0].name, "half dozen glazed donuts");
    assert_eq!(list[0].price, 625);
    assert_eq!(list[0].tax, 1.0);
    assert_eq!(list[0].type_, "food");

    assert_eq!(list[1].name, "dozen glazed donuts");
    assert_eq!(list[1].price, 1099);
    assert_eq!(list[1].tax, 1.0);
    assert_eq!(list[1].type_, "food");

    assert_eq!(list[2].name, "small chocolate milk");
    assert_eq!(list[2].price, 169);
    assert_eq!(list[2].tax, 1.0);
    assert_eq!(list[2].type_, "drink");

    assert_eq!(list[3].name, "large chocolate milk");
    assert_eq!(list[3].price, 249);
    assert_eq!(list[3].tax, 1.0);
    assert_eq!(list[3].type_, "drink");

    assert_eq!(list[4].name, "energy drink");
    assert_eq!(list[4].price, 300);
    assert_eq!(list[4].tax, 1.25);
    assert_eq!(list[4].type_, "drink");

    assert_eq!(list[5].name, "painting one");
    assert_eq!(list[5].price, 5000);
    assert_eq!(list[5].tax, 1.0);
    assert_eq!(list[5].type_, "other");

    assert_eq!(list[6].name, "painting two");
    assert_eq!(list[6].price, 10000);
    assert_eq!(list[6].tax, 1.0);
    assert_eq!(list[6].type_, "other");

    assert_eq!(list[7].name, "painting three");
    assert_eq!(list[7].price, 7500);
    assert_eq!(list[7].tax, 1.0);
    assert_eq!(list[7].type_, "other");
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
