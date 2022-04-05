use crate::app::{model::Database, web::WebServer};
use colored::Colorize;
use config::{Config, ConfigError, File};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use std::fs::File as StdFile;
use std::io::Write;

mod model;
mod web;

use web::Error as WebError;

#[derive(Debug, Deserialize)]
pub struct App {
    pub database: Database,
    pub webserver: WebServer,
}

impl App {
    pub fn new() -> Result<Self, ConfigError> {
        let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/config");
        let pos_toml = &format!("{}/{}", path, "pos_config.toml");
        let default_toml = &format!("{}/{}", path, ".defaults/POS_DEFAULTS.toml");

        let s = Config::builder()
            .add_source(File::with_name(default_toml))
            .add_source(File::with_name(pos_toml))
            .build()?;

        let app: App = s.try_deserialize()?;

        Ok(app)
    }

    fn generate_env_file(&self) {
        let body = format!("DATABASE_URL={}", self.database.file_path);
        let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/.env");
        let mut file = StdFile::create(path).unwrap();
        file.write_all(body.as_bytes()).unwrap();
    }

    pub async fn run(self) -> Result<(), WebError> {
        print!("Generating .env file ... ");
        self.generate_env_file();
        println!("{}", "done".green());

        print!("Starting server ... ");
        let connection = self.database.establish_db_conn();

        self.webserver.establish_webserver(connection).await
    }
}
