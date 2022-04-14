use crate::app::App;
use crate::schema::items::dsl::items;
use crate::schema::purchases::dsl::purchases;
use diesel::RunQueryDsl;

#[actix_rt::test]
async fn model_db_purchase() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();
    let all_purchases = purchases.load::<(i32, String, String, i32)>(&conn).unwrap();

    assert_eq!(all_purchases.len(), 3, "number of seed purchases");
}

#[actix_rt::test]
async fn model_db_item() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let all_items = items.load::<(String, i32, f32, String)>(&conn).unwrap();

    assert_eq!(all_items.len(), 3, "number of seed items");
}
