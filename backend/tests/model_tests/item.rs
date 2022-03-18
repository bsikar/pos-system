use super::ItemMac;
use crate::app::model::db::init_db;
use crate::app::model::item::Item;
use crate::app::model::{self, Database};

#[tokio::test]
async fn model_item_create_ok() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    let item = Item {
        name: "single donut hole".to_string(),
        price: 30,
    };

    // action
    let item_created = ItemMac::create(&db, item).await?;

    // check
    assert_eq!(item_created.name, "single donut hole");
    assert_eq!(item_created.price, 30);

    Ok(())
}

#[tokio::test]
async fn model_item_create_duplicate() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    let item = Item {
        name: "single glazed donut".to_string(),
        price: 220,
    };

    // action
    let result = ItemMac::create(&db, item).await;

    // check
    match result {
        Ok(_) => panic!("Expected error"),
        Err(model::Error::ItemAlreadyExists(name)) => {
            assert_eq!(name, "single glazed donut");
        }
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_item_create_invalid_price() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    let item = Item {
        name: "single donut hole".to_string(),
        price: -30,
    };

    // action
    let result = ItemMac::create(&db, item).await;

    // check
    match result {
        Ok(_) => panic!("Expected error"),
        Err(model::Error::InvalidItemPrice(price)) => {
            assert_eq!(price, -30);
        }
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_item_create_invalid_name() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    let item = Item {
        name: "".to_string(),
        price: 0,
    };

    // action
    let result = ItemMac::create(&db, item).await;

    // check
    match result {
        Ok(_) => panic!("Expected error"),
        Err(model::Error::EmptyItemName) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_item_get_wrong_name() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let result = ItemMac::get(&db, "none existant item".to_string()).await;

    match result {
        Ok(_) => {}
        Err(model::Error::EntityNotFound(type_, id)) => {
            assert_eq!(type_, "item");
            assert_eq!(id, "none existant item");
        }
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_item_list() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let items = ItemMac::list(&db).await?;

    // check
    assert_eq!(items.len(), 3, "number of seed items");

    // single glazed donut
    assert_eq!(items[0].name, "single glazed donut");
    assert_eq!(items[0].price, 120);

    // half dozen glazed donuts
    assert_eq!(items[1].name, "half dozen glazed donuts");
    assert_eq!(items[1].price, 625);

    // dozen glazed donuts
    assert_eq!(items[2].name, "dozen glazed donuts");
    assert_eq!(items[2].price, 1099);

    Ok(())
}

#[tokio::test]
async fn model_item_delete() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let item = ItemMac::delete(&db, "single glazed donut".to_string()).await?;

    // check
    assert_eq!(item.name, "single glazed donut");
    assert_eq!(item.price, 120);

    // check list
    let list = ItemMac::list(&db).await?;
    assert_eq!(list.len(), 2, "number of items");

    Ok(())
}

#[tokio::test]
async fn model_item_update() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let name = "single glazed donut".to_string();
    let db = init_db().await?;
    let item = Item {
        name: name.clone(),
        price: 999,
    };

    // action
    let item_updated = ItemMac::update(&db, name, item).await?;

    // check
    assert_eq!(item_updated.name, "single glazed donut");
    assert_eq!(item_updated.price, 999);

    Ok(())
}
