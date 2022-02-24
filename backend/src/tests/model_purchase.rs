use crate::model;
use crate::model::db::init_db;
use serde_json::Value as JsonValue;
use serde_json::{self, json};

use super::PurchaseMac;
use super::PurchasePatch;

#[tokio::test]
async fn model_purchase_create() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;
    let data_fx = PurchasePatch {
        items: Some(json!([])),
        ..Default::default()
    };

    // action
    let purchase_created = PurchaseMac::create(&db, data_fx.clone()).await?;

    // check
    assert_eq!(purchase_created.items, json!({"items": []}));
    assert_eq!(purchase_created.total, 0);
    assert!(purchase_created.id >= 1000, "id should be >= 1000");

    Ok(())
}

#[tokio::test]
async fn model_purchase_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let purchase = PurchaseMac::get(&db, 100).await?;

    // check
    let raw_json = r#"{ "items": [{"name": "test 1", "price": 100, "quantity" : 1}] }"#;
    let json: JsonValue = serde_json::from_str(raw_json).unwrap();
    assert_eq!(purchase.id, 100);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 100);

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
    assert_eq!(purchases.len(), 2, "number of seed purchases");

    let json = json!([
        { "items": [{"name": "test 2", "price": 200, "quantity" : 2}] },
        { "items": [{"name": "test 1", "price": 100, "quantity" : 1}] },
    ]);
    // purchase 101
    assert_eq!(purchases[0].id, 101);
    assert_eq!(purchases[0].items, json[0]);
    assert_eq!(purchases[0].total, 400);

    // purchase 100
    assert_eq!(purchases[1].id, 100);
    assert_eq!(purchases[1].items, json[1]);
    assert_eq!(purchases[1].total, 100);

    Ok(())
}

#[tokio::test]
async fn model_purchase_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    // fixture
    let db = init_db().await?;

    // action
    let purchase = PurchaseMac::delete(&db, 100).await?;

    // check - deleted item
    let json = json!({ "items": [{"name": "test 1", "price": 100, "quantity" : 1}] });
    assert_eq!(purchase.id, 100);
    assert_eq!(purchase.items, json);
    assert_eq!(purchase.total, 100);

    // check - list
    let list = PurchaseMac::list(&db).await?;
    assert_eq!(1, list.len());

    Ok(())
}

#[tokio::test]
#[ignore]
async fn model_purchase_update() -> Result<(), Box<dyn std::error::Error>> {
    // TODO FIX
    Ok(())
}
