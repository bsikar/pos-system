use crate::app::{model::Database, web::WebServer};
use config::{Config, ConfigError, File};
use diesel::result::Error as DieselError;
use serde::Deserialize;

mod model;
mod web;

use web::Error as WebError;

#[derive(Debug, Deserialize)]
pub struct App {
    database: Database,
    webserver: WebServer,
}

impl App {
    pub fn new() -> Result<Self, ConfigError> {
        let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/config");
        let db_toml = &format!("{}/{}", path, "database.toml");
        let web_toml = &format!("{}/{}", path, "webserver.toml");
        let default_toml = &format!("{}/{}", path, ".defaults/POS_DEFAULTS.toml");

        let s = Config::builder()
            .add_source(File::with_name(db_toml))
            .add_source(File::with_name(web_toml))
            .add_source(File::with_name(default_toml))
            .build()?;

        let app: App = s.try_deserialize()?;

        Ok(app)
    }

    pub async fn run(self) -> Result<(), WebError> {
        println!("Starting server...");
        let connection = self.database.establish_db_conn();

        self.webserver.establish_webserver(connection).await
    }
}
