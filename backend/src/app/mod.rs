use crate::app::model::{init_db, Db};
use crate::app::web::{actix, warp};

mod model;
mod web;

pub struct App {
    web_folder: String,
    web_port: u16,
    web_framework: String,
    db: Db,
    tax: f32,
}

impl App {
    pub async fn new(web_folder: String, web_port: u16, web_framework: String, tax: f32) -> Self {
        let db = init_db().await.expect("Could not initialize database");

        Self {
            web_folder,
            web_port,
            web_framework,
            db,
            tax,
        }
    }

    pub async fn run(self) -> Result<(), crate::app::web::Error> {
        match self.web_framework.as_str() {
            "actix" => actix::start_web(self.web_folder, self.web_port, self.db, self.tax).await,
            "warp" => {
                warp::start_web(
                    &self.web_folder.to_string(),
                    self.web_port,
                    self.db,
                    self.tax,
                )
                .await
            }
            _ => unreachable!(),
        }
    }
}
