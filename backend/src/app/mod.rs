use config::{Config, ConfigError, File};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;

mod model;

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

        let app: App = s.try_deserialize()?;

        Ok(app)
    }

    pub async fn run(&self) {
        let connection = self.establish_db_conn();
    }

    pub fn establish_db_conn(&self) -> PgConnection {
        let database_url = format!(
            "postgres://{}:{}@{}/{}",
            self.database.user, self.database.pwd, self.database.net_id, self.database.db_name
        );

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
