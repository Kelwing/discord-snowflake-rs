use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Error getting current timestamp")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("Error parsing integer")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;