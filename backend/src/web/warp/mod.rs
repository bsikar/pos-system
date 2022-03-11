use crate::model::{self, Db};
use crate::web;
use crate::web::warp::item::item_rest_filters;
use crate::web::warp::purchase::purchase_rest_filters;
use crate::web::warp::Error::FailStartWebFolderNotFound;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;
use std::convert::Infallible;
use std::path::Path;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

mod item;
mod purchase;

#[async_trait]
pub trait WebService<I, D> {
    async fn list(db: Arc<Db>) -> Result<Json, warp::Rejection>;
    async fn get(db: Arc<Db>, id: I) -> Result<Json, warp::Rejection>;
    async fn create(db: Arc<Db>, data: D) -> Result<Json, warp::Rejection>;
    async fn update(db: Arc<Db>, id: I, data: D) -> Result<Json, warp::Rejection>;
    async fn delete(db: Arc<Db>, id: I) -> Result<Json, warp::Rejection>;

    fn json_response<T: Serialize + std::fmt::Debug>(data: T) -> Result<Json, warp::Rejection> {
        let response = json!({ "data": data });
        Ok(warp::reply::json(&response))
    }
}

pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Db>) -> Result<(), web::Error> {
    // validate web_folder
    if !Path::new(web_folder).exists() {
        return Err(web::Error::WarpError(FailStartWebFolderNotFound(
            web_folder.to_string(),
        )));
    }

    // apis
    let api_purchases = purchase_rest_filters(db.clone());
    let api_items = item_rest_filters(db);
    let apis = api_purchases.or(api_items);

    // static content
    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    let static_site = content.or(root_index).recover(handle_rejection);

    // combine all routes
    let routes = apis.or(static_site);

    println!(
        "Started on 0.0.0.0:{} with web_folder: {} and using warp",
        web_port, web_folder
    );

    warp::serve(routes).run(([0, 0, 0, 0], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    // print to server side
    eprintln!("ERROR - {:?}", err);

    let result = warp::reply::html(format!("Error {:?}", err));

    Ok(warp::reply::with_status(
        result,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String),
}

#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage { typ, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{}", other))
    }
}

impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection("model::Error", format!("{}", other))
    }
}
