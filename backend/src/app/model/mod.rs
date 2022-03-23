use diesel::result::Error as DieselError;
use thiserror::Error as ThisError;

mod item;
mod purchase;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    DieselError(#[from] DieselError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Item Already Exists - {0}")]
    ItemAlreadyExists(String),

    #[error("Item Not Found - {0}")]
    ItemNotFound(String),

    #[error("Invalid Item Price - {0}")]
    InvalidItemPrice(i64),

    #[error("Invalid Item Name- {0}")]
    InvalidItemName(String),

    #[error("Empty Items")]
    EmptyItems,

    #[error("Empty Item Name")]
    EmptyItemName,

    #[error("Purchase Not Found - {0}")]
    PurchaseNotFound(String),
}
