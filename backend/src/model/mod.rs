use thiserror::Error as ThisError;

pub mod db;

pub use db::Db;

mod purchase;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
