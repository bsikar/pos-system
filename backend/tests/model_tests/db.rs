use crate::app::model::item::Item;
use crate::app::model::purchase::Purchase;
use crate::app::App;
use diesel::RunQueryDsl;

#[actix_rt::test]
async fn model_db_purchase() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();
    let result: Vec<Purchase> = diesel::sql_query("SELECT * from purchases")
        .load(&conn)
        .unwrap();

    assert_eq!(result.len(), 3, "number of seed purchases");
}

#[actix_rt::test]
async fn model_db_item() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();
    let result: Vec<Item> = diesel::sql_query("SELECT * from items")
        .load(&conn)
        .unwrap();

    assert_eq!(result.len(), 3, "number of seed items");
}
