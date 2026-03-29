use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpecanError {
    #[error("connection error: {0}")]
    Connection(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(#[from] std::num::ParseFloatError),

    #[error("instrument error: {0}")]
    Instrument(String),
}
