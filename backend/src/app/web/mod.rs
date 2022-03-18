use thiserror::Error as ThisError;

pub mod actix;
pub mod warp;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    WarpError(#[from] warp::Error),

    #[error(transparent)]
    ActixError(#[from] actix_web::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
