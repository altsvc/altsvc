use crate::error::ParseError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Service {
    pub clear: bool,
    pub protocol_id: Option<String>,
    pub alt_authority: Option<AltAuthority>,
    pub max_age: Option<i32>,
    pub persist: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct AltAuthority {
    pub host: Option<String>,
    pub port: Option<String>,
}

impl Service {
    pub fn parse(s: &str) -> Result<Vec<Service>, ParseError> {
        // Implementation here
        Ok(vec![])
    }
}

