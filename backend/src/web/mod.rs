use crate::model::Db;
use thiserror::Error as ThisError;

mod actix;
mod warp;

pub async fn start_web(
    web_folder: &str,
    web_port: u16,
    web_framework: &str,
    db: Db,
) -> Result<(), Error> {
    match web_framework {
        "warp" => warp::start_web(web_folder, web_port, db).await,
        "actix" => actix::start_web(web_folder.to_string(), web_port, db).await,
        _ => unreachable!(),
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    WarpError(#[from] warp::Error),

    #[error(transparent)]
    ActixError(#[from] actix_web::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
