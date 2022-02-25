use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    // action
    let db = init_db().await?;

    // check
    let result = sqlx::query("SELECT * from purchase").fetch_all(&db).await?;
    assert_eq!(result.len(), 3, "number of seed purchases");

    Ok(())
}
