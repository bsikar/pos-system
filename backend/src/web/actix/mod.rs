use crate::web::Db;
use crate::web::Error;
use std::sync::Arc;

pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Db>) -> Result<(), Error> {
    Ok(())
}
