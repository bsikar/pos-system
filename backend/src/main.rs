#![allow(clippy::enum_variant_names)]

#[macro_use]
extern crate diesel;

mod app;
mod schema;
use app::App;

#[actix_web::main]
async fn main() {
    let app = App::new().unwrap();

    app.run().await;
}
