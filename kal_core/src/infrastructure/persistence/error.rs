use thiserror::Error;

#[derive(Debug, Error)]
pub enum MapperError {
    #[error("Invalid ID: {0}")]
    InvalidId(String),

    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid enum value: {0}")]
    InvalidEnum(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),
}
