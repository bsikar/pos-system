mod model;
mod web;

// database
const db_net_id: &str = "0.0.0.0";
const db_port: u16 = 5432;
const db_max_connections: u8 = 5;
const db_root_name: &str = "postgres";
const db_root_user: &str = "postgres";
const db_root_pwd: &str = "postgres";
const db_name: &str = "postgres";
const db_user: &str = "pos_user";
const db_pwd: &str = "pos_user";

// web
const web_net_id: &str = "0.0.0.0";
const web_port: u16 = 8080;
const web_folder: &str = "../frontend/web-folder/";

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    net_id: &str,
    port: u16,
    root_db: &str,
    root_user: &str,
    root_pwd: &str,
    db_name: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct WebServer {
    net_id: &str,
    port: u16,
    folder: &str,
}

pub struct App {
    db: Database,
    web: WebServer,
}

impl App {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/database.toml"))
            .add_source(File::with_name("config/webserver.toml"))
            .add_source(Environment::with_prefix("POS"))
            .build?;

        s.try_deserialize()
    }
}
