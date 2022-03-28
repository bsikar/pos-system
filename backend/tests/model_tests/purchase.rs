use crate::app::model::purchase::Purchase;
use crate::app::model::Error as ModelError;
use crate::app::App;
use serde_json::json;

#[actix_rt::test]
async fn model_purchase_create_err_1() {
    let conn = App::new()
        .unwrap()
        .database
        .establish_db_conn()
        .get()
        .unwrap();

    let item = json!({});

    let result = Purchase::create(&conn, item);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(ModelError::EmptyItems) => {}
        other_err => panic!("unexpected error: {:?}", other_err),
    }
}
