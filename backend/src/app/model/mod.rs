use crate::app::DieselError;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use serde::Deserialize;
use thiserror::Error as ThisError;

pub mod item;
pub mod purchase;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub net_id: String,
    pub port: u16,
    pub max_connections: u32,
    pub root_db_name: String,
    pub root_user: String,
    pub root_pwd: String,
    pub db_name: String,
    pub user: String,
    pub pwd: String,
    pub file_path: String,
}

impl Database {
    pub fn establish_db_conn(&self) -> DbPool {
        let migr = ConnectionManager::<SqliteConnection>::new(&self.file_path);
        r2d2::Pool::builder()
            .build(migr)
            .expect("could not build connection pool")
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    // Diesel
    #[error(transparent)]
    DieselError(#[from] DieselError),

    #[error(transparent)]
    DieselConnectionError(#[from] diesel::ConnectionError),

    // purchase
    #[error("Purchase Not Found {0}")]
    PurchaseNotFound(i32),

    // Item
    #[error("Item Not Found - {0}")]
    ItemNotFound(String),

    #[error("Invalid Item Price - {0}")]
    InvalidItemPrice(i32),

    #[error("Item Already Exists - {0}")]
    ItemAlreadyExists(String),

    #[error("Empty Item Name")]
    EmptyItemName,

    #[error("Empty Items")]
    EmptyItems,
}

#[cfg(test)]
#[path = "../../../tests/model_tests/db.rs"]
mod model_tests;
