use crate::app::model::{DbPool, Error as ModelError};
use actix_files::Files;
use actix_web::HttpResponse;
use actix_web::{web::Data, App, HttpServer};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

mod item;
mod purchase;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebServer {
    pub net_id: String,
    pub port: u16,
    pub folder: String,
}

impl WebServer {
    pub async fn establish_webserver(self, connection: DbPool) -> Result<(), Error> {
        let folder = self.folder.clone();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(Data::new(connection.clone()))
                .configure(item::item_rest_filters)
                .configure(purchase::purchase_rest_filters)
                .service(Files::new("/", folder.clone()).index_file("index.html"))
        })
        .bind((self.net_id.clone(), self.port))?;

        println!(
            "Starting server on {}:{} with folder: {}",
            self.net_id, self.port, self.folder
        );
        server.run().await.map_err(Error::from)
    }
}

pub fn handle_result<T: Serialize>(result: Result<T, ModelError>) -> HttpResponse {
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    ActixError(#[from] actix_web::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
