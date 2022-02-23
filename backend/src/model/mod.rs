use thiserror::Error as ThisError;

mod db;
mod purchase;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
