use thiserror::Error as ThisError;

pub mod db;

use async_trait::async_trait;
pub use db::init_db;
pub use db::Db;
pub use purchase::{Purchase, PurchaseMac, PurchasePatch};

pub mod item;
pub mod purchase;

#[async_trait]
pub trait Database<R, D, I> {
    async fn create(db: &Db, data: D) -> Result<R, Error>;
    async fn get(db: &Db, _: I) -> Result<R, Error>;
    async fn update(db: &Db, _: I, data: D) -> Result<R, Error>;
    async fn list(db: &Db) -> Result<Vec<R>, Error>;
    async fn delete(db: &Db, _: I) -> Result<R, Error>;

    fn handle_fetch_one_result(
        result: Result<R, sqlx::Error>,
        typ: &'static str,
        id: I,
    ) -> Result<R, Error>
    where
        I: std::fmt::Display,
    {
        result.map_err(|sqlx_error| match sqlx_error {
            sqlx::Error::RowNotFound => Error::EntityNotFound(typ, format!("{}", id)),
            other => Error::SqlxError(other),
        })
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Item Already Exists - {0}")]
    ItemAlreadyExists(String),
}
