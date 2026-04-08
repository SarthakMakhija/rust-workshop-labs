use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CacheError {
    ShuttingDown,
}

impl Display for CacheError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for CacheError {}