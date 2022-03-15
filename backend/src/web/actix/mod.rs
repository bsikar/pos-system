use crate::web::{Db, Error};
use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};

use std::sync::Arc;

mod item;
mod purchase;

pub async fn start_web(web_folder: String, web_port: u16, db: Arc<Db>) -> Result<(), Error> {
    let folder = web_folder.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .configure(item::item_rest_filters)
            .configure(purchase::purchase_rest_filters)
            .service(Files::new("/", &web_folder).index_file("index.html"))
    })
    .bind(("0.0.0.0", web_port))?;

    println!(
        "Started on 0.0.0.0:{} with web_folder: {} and using actix",
        web_port, folder
    );

    let result = server.run().await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::IOError(e)),
    }
}
