use crate::model::db::init_db;
use serde_json::json;

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
    assert_eq!(purchase_created.items, json!([]));
    assert_eq!(purchase_created.total, 0);
    assert!(purchase_created.id >= 1000, "id should be >= 1000");

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
