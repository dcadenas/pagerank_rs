use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum PagerankError {
    CapacityError(String),
}

impl Display for PagerankError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PagerankError::CapacityError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for PagerankError {}
