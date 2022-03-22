use crate::app::web::start_web;
use config::{Config, ConfigError, File};
use serde::Deserialize;

mod model;
mod web;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    net_id: String,
    port: u16,
    max_connections: u32,
    root_db_name: String,
    root_user: String,
    root_pwd: String,
    db_name: String,
    user: String,
    pwd: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct WebServer {
    net_id: String,
    port: u16,
    folder: String,
}

#[derive(Debug, Deserialize)]
pub struct App {
    database: Database,
    webserver: WebServer,
}

impl App {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/database.toml"))
            .add_source(File::with_name("config/webserver.toml"))
            .add_source(File::with_name("config/.defaults/POS_DEFAULTS.toml"))
            .build()?;

        s.try_deserialize()
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db = model::db::init_db().await?;

        match start_web(self.webserver.folder.clone(), self.webserver.port, db).await {
            Ok(_) => println!("Server ended"),
            Err(e) => eprintln!("ERROR - web server failed to start. Cause {:?}", e),
        }

        Ok(())
    }
}
