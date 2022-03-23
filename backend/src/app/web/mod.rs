use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use serde::Deserialize;
use thiserror::Error as ThisError;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebServer {
    pub net_id: String,
    pub port: u16,
    pub folder: String,
}

impl WebServer {
    pub async fn establish_webserver(
        self,
        connection: r2d2::Pool<ConnectionManager<PgConnection>>,
    ) -> Result<(), Error> {
        let folder = self.folder.clone();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(Data::new(connection.clone()))
                //.configure(item::item_rest_filters)
                //.configure(purchase::purchase_rest_filters)
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

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    ActixError(#[from] actix_web::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
