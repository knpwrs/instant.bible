use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct DataInputError {}

impl DataInputError {
    pub fn new() -> Self {
        DataInputError {}
    }
}

impl Display for DataInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot convert string to index data")
    }
}

impl Error for DataInputError {
    fn description(&self) -> &str {
        "Cannot convert string to index data"
    }
}