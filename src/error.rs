use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpecanError {
    #[error("connection error: {0}")]
    Connection(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("parser error: {0}")]
    Parser(String),
}
