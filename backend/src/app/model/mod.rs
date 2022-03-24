use crate::app::DieselError;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use serde::Deserialize;
use thiserror::Error as ThisError;

pub mod item;
pub mod purchase;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
}

impl Database {
    pub fn establish_db_conn(&self) -> DbPool {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pwd, self.net_id, self.port, self.db_name
        );

        let migr = ConnectionManager::<PgConnection>::new(url);
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

    // Item
    #[error("Item Not Found - {0}")]
    ItemNotFound(String),

    #[error("Invalid Item Price - {0}")]
    InvalidItemPrice(i64),

    #[error("Item Already Exists - {0}")]
    ItemAlreadyExists(String),

    #[error("Empty Item Name")]
    EmptyItemName,

    #[error("Empty Items")]
    EmptyItems,
}
