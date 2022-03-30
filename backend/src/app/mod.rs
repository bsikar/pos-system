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

    fn generate_docker_compose_yml(&self) {
        let body = format!(
            "version: '3.3'
services:
    postgres:
        container_name: pos-pg
        volumes:
            - ./postgres-data/:/var/lib/postgresql/data
        ports:
            - '{0}:{0}'
        environment:
            - POSTGRES_USER={1}
            - POSTGRES_PASSWORD={2}
            - POSTGRES_DB={3}
        image: 'postgres:14'\n",
            self.database.port, self.database.user, self.database.pwd, self.database.db_name
        );
        std::env::set_var("POSTGRES_USER", self.database.user.clone());
        std::env::set_var("POSTGRES_PASSWORD", self.database.pwd.clone());
        std::env::set_var("POSTGRES_DB", self.database.db_name.clone());
        let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../docker-compose.yml");
        let mut file = StdFile::create(path).unwrap();
        file.write_all(body.as_bytes()).unwrap();
    }

    fn start_docker(&self) {
        if cfg!(windows) {
            std::process::Command::new("cmd")
                .args(&["/C", "docker-compose up -d"])
                .output()
                .expect("failed to execute process");
        } else {
            std::process::Command::new("sh")
                .args(&["-c", "docker-compose up -d"])
                .output()
                .expect("failed to execute process");
        }
    }

    fn generate_env_file(&self) {
        let body = format!(
            "DATABASE_URL=postgres://{}:{}@{}/{}",
            self.database.user, self.database.pwd, self.database.net_id, self.database.db_name
        );
        println!("{}", self.database.net_id.green());
        let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/.env");
        let mut file = StdFile::create(path).unwrap();
        file.write_all(body.as_bytes()).unwrap();
    }

    pub async fn run(self) -> Result<(), WebError> {
        print!("Generating docker-compose.yml ... ");
        self.generate_docker_compose_yml();
        println!("{}", "done".green());

        print!("Generating .env file ... ");
        self.generate_env_file();
        println!("{}", "done".green());

        print!("Starting docker ... ");
        self.start_docker();
        println!("{}", "done".green());

        print!("Starting server ... ");
        let connection = self.database.establish_db_conn();

        self.webserver.establish_webserver(connection).await
    }
}
