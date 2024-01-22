use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("invalid value of 'ma': {0}")]
    InvalidMaValue(String),

    #[error("invalid value of 'persist': {0}")]
    InvalidPersistValue(String),

    #[error("cannot unquote the value of 'alt-authority': {0}")]
    UnquoteAltAuthorityError(String),

    #[error("invalid value of 'alt-authority': {0}")]
    InvalidAltAuthorityValue(String),
}
