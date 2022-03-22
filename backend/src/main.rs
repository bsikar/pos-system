#![allow(clippy::enum_variant_names)]

#[macro_use]
extern crate diesel;

mod app;
mod schema;
use app::App;

#[tokio::main]
async fn main() {
    let app = App::new().unwrap();

    app.run().await;
}
