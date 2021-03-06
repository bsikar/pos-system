use crate::app::model::{DbPool, Error as ModelError};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::HttpResponse;
use actix_web::{web::Data, App, HttpServer};
use colored::Colorize;
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
                .wrap(Cors::default().allowed_origin("http://localhost:3000"))
                .app_data(Data::new(connection.clone()))
                .configure(item::item_rest_filters)
                .configure(purchase::purchase_rest_filters)
                .service(Files::new("/", folder.clone()).index_file("index.html"))
        })
        .bind((self.net_id.clone(), self.port))?;

        let msg = format!(
            "Started server on {}:{} with folder: {}",
            self.net_id, self.port, self.folder
        );

        println!("{}", msg.green());
        server.run().await.map_err(Error::from)
    }
}

pub fn handle_result<T: Serialize>(result: Result<T, ModelError>) -> HttpResponse {
    match result {
        Ok(item) => {
            let mut json = serde_json::to_string(&item).unwrap();
            json = json.replace(r#""type_":"#, r#""type":"#);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
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
