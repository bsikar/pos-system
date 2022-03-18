#![allow(clippy::enum_variant_names)]

#[macro_use]
extern crate actix_web;

mod app;

use crate::app::App;
use clap::Arg;

const DEFAULT_WEB_FOLDER: &str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 3030;
const DEFAULT_FRAMEWORK: &str = "actix";
const DEFAULT_TAX: &str = "6.25%";

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
        .arg(
            Arg::new("framework")
                .help("This is the web framework to use")
                .value_name("WEB_FRAMEWORK")
                .short('f')
                .long("framework")
                .possible_values(&["warp", "actix"])
                .default_value(DEFAULT_FRAMEWORK),
        )
        .arg(
            Arg::new("tax")
                .help("This is the tax percent to use")
                .value_name("TAX_RATE")
                .short('t')
                .long("tax")
                .default_value(DEFAULT_TAX),
        )
        .get_matches();

    let web_framework = matches
        .value_of("framework")
        .expect("Could not parse web framework");
    let web_port = matches
        .value_of("port")
        .map(|s| s.parse::<u16>().expect("Could not parse port"))
        .unwrap();
    let web_folder = matches.value_of("folder").expect("Could not parse folder");
    let tax = matches
        .value_of("tax")
        .map(|s| {
            if s.contains('%') {
                s.trim_end_matches('%')
                    .parse::<f32>()
                    .expect("Could not parse tax rate")
            } else {
                s.parse::<f32>().expect("Could not parse tax rate")
            }
        })
        .unwrap();

    let app = App::new(
        web_folder.to_string(),
        web_port,
        web_framework.to_string(),
        tax,
    )
    .await;
    match app.run().await {
        Ok(_) => println!("Server ended"),
        Err(e) => eprintln!("ERROR - web server failed to start. Cause {:?}", e),
    };
}
