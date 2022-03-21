#![allow(clippy::enum_variant_names)]

#[macro_use]
extern crate actix_web;

mod app;
use app::App;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    match app.run().await {
        Ok(_) => println!("Server ended"),
        Err(e) => eprintln!("ERROR - web server failed to start. Cause {:?}", e),
    }
}
