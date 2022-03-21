#![allow(clippy::enum_variant_names)]

#[macro_use]
extern crate actix_web;

mod model;
mod web;

use crate::model::init_db;
use crate::web::start_web;
use clap::Arg;

const DEFAULT_WEB_FOLDER: &str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 3030;

#[tokio::main]
async fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("port")
                .help("This is the port to run the web server on")
                .value_name("WEB_PORT")
                .short('p')
                .long("port")
                .default_value(&format!("{}", DEFAULT_WEB_PORT)),
        )
        .arg(
            Arg::new("folder")
                .help("This is the web folder to serve")
                .value_name("WEB_FOLDER")
                .short('i')
                .long("folder")
                .default_value(DEFAULT_WEB_FOLDER),
        )
        .get_matches();

    let web_port = matches
        .value_of("port")
        .map(|s| s.parse::<u16>().expect("Could not parse port"))
        .unwrap();
    let web_folder = matches.value_of("folder").expect("Could not parse folder");

    // get the database
    let db = init_db().await.expect("Cannot init db");

    // start the server
    match start_web(web_folder.to_string(), web_port, db).await {
        Ok(_) => println!("Server ended"),
        Err(e) => eprintln!("ERROR - web server failed to start. Cause {:?}", e),
    }
}
