#![allow(clippy::enum_variant_names)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate actix_web;

mod app;
mod schema;
use app::App;

#[actix_web::main]
async fn main() {
    let app = App::new().unwrap();

    match app.run().await {
        Ok(_) => println!("Server stopped"),
        Err(e) => {
            eprintln!("Server failed to start: {}", e);
            std::process::exit(1);
        }
    }
}
