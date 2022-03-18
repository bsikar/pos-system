use crate::app::model::db::init_db;
use crate::app::model::{self, Database};
use serde_json::{self, json};

use super::PurchaseMac;
use super::PurchasePatch;
use crate::app::model::purchase::calculate_total;

#[tokio::test]
async fn model_purchase_create_err_1() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let data_fx = PurchasePatch { items: json!({}) };

    // action
    let result = PurchaseMac::create(&db, data_fx).await;

    // check result
    match result {
        Ok(_) => panic!("Expected error"),
        Err(model::Error::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_purchase_create_err_2() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let data_fx = PurchasePatch { items: json!([]) };

    // action
    let result = PurchaseMac::create(&db, data_fx).await;

    // check result
    match result {
        Ok(_) => panic!("Expected error"),
        Err(model::Error::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_purchase_create_err_3() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let data_fx = PurchasePatch { items: json!(null) };

    // action
    let result = PurchaseMac::create(&db, data_fx).await;

    // check result
    match result {
        Ok(_) => panic!("Expected error"),
        Err(model::Error::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_purchase_create_not_in_database() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let json = json!([{"name": "some random item", "price": 120, "quantity" : 1}]);
    let data_fx = PurchasePatch { items: json };

    // action
    let result = PurchaseMac::create(&db, data_fx).await;

    // check result
    match result {
        Ok(_) => panic!("Expected error"),
        Err(model::Error::InvalidItemName(name)) => {
            assert_eq!(name, "some random item")
        }
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_purchase_create_ok_1() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity" : 1}]);
    let data_fx = PurchasePatch {
        items: json.clone(),
    };

    // action
    let purchase_created = PurchaseMac::create(&db, data_fx).await?;

    // check
    assert_eq!(purchase_created.items, json);
    assert_eq!(purchase_created.total, 120);
    assert!(purchase_created.id >= 1000, "id should be >= 1000");

    Ok(())
}

#[tokio::test]
async fn model_purchase_create_ok_2() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity" : 1}, {"name": "half dozen glazed donuts", "price": 625, "quantity" : 1}]);
    let data_fx = PurchasePatch {
        items: json.clone(),
    };

    // action
    let purchase_created = PurchaseMac::create(&db, data_fx).await?;

    // check
    assert_eq!(purchase_created.items, json);
    assert_eq!(purchase_created.total, 745);
    assert!(purchase_created.id >= 1000, "id should be >= 1000");

    Ok(())
}

#[tokio::test]
async fn model_purchase_get_ok_1() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let purchase = PurchaseMac::get(&db, 100).await?;

    // check
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity" : 1}]);
    assert_eq!(purchase.id, 100);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 120);

    Ok(())
}

#[tokio::test]
async fn model_purchase_get_ok_2() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let purchase = PurchaseMac::get(&db, 102).await?;

    // check
    let json = json!([
        {"name": "half dozen glazed donuts", "price": 625, "quantity" : 1},
        {"name": "dozen glazed donuts", "price": 1099, "quantity" : 2},
    ]);

    assert_eq!(purchase.id, 102);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 2823);

    Ok(())
}

#[tokio::test]
async fn model_purchase_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let result = PurchaseMac::get(&db, 10).await;

    match result {
        Ok(_) => {}
        Err(model::Error::EntityNotFound(type_, id)) => {
            assert_eq!(type_, "purchase");
            assert_eq!(id, "10");
        }
        other_err => panic!("unexpected error: {:?}", other_err),
    }

    Ok(())
}

#[tokio::test]
async fn model_purchase_list() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let purchases = PurchaseMac::list(&db).await?;

    // check
    assert_eq!(purchases.len(), 3, "number of seed purchases");

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

    Ok(())
}

#[tokio::test]
async fn model_purchase_delete() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let purchase = PurchaseMac::delete(&db, 100).await?;

    // check - deleted item
    let json = json!([{"name": "single glazed donut", "price": 120, "quantity" : 1}]);
    assert_eq!(purchase.id, 100);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 120);

    // check - list
    let list = PurchaseMac::list(&db).await?;
    assert_eq!(list.len(), 2);

    Ok(())
}

#[tokio::test]
async fn model_purchase_update() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let data_fx = PurchasePatch { items: json!([]) };

    // action
    let purchase = PurchaseMac::update(&db, 100, data_fx).await?;

    // check
    assert_eq!(purchase.items, json!([]));
    assert_eq!(purchase.total, 0);
    assert_eq!(purchase.id, 100);

    Ok(())
}

#[tokio::test]
async fn model_purchase_calculate_total_1() -> Result<(), Box<dyn std::error::Error>> {
    let json = json!([{"name": "single donut hole", "price": 30, "quantity": 1}]);

    assert_eq!(calculate_total(&json), 30);

    Ok(())
}

#[tokio::test]
async fn model_purchase_calculate_total_2() -> Result<(), Box<dyn std::error::Error>> {
    let json = json!([{"name": "single donut hole", "price": 30, "quantity": 2}, {"name": "dozen donut holes", "price": 200, "quantity": 1}]);

    assert_eq!(calculate_total(&json), 260);

    Ok(())
}
