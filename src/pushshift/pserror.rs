#[warn(clippy::all)]
use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};
use url::ParseError;

use csv::Error as CSVError;
use serde_json::Error as SerdeJSONError;

pub static MAX_PS_FETCH_SIZE: u32 = 1000;

#[derive(Debug)]
pub enum PSError {
    AlreadyAdded(String),
    Csv(CSVError),
    InvalidSubreddit(String),
    NoParams,
    Parse(ParseError),
    Reqwest(reqwest::Error),
    SerdeJson(SerdeJSONError),
    SizeTooHigh(u32),
}

impl Display for PSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PSError::*;
        match self {
            AlreadyAdded(param) => write!(f, "Parameter already added: {}", param),
            Csv(error) => write!(f, "CSV: {}", error.to_string()),
            InvalidSubreddit(subreddit) => write!(
                f,
                "Subreddit may only contain alphanumeric and _: {}",
                subreddit
            ),
            NoParams => write!(
                f,
                "No parameters found. You have to specify parameters such as a subreddit."
            ),
            Parse(error) => write!(f, "Parse: {}", error.to_string()),
            Reqwest(error) => write!(f, "Reqwest: {}", error.to_string()),
            SerdeJson(error) => write!(f, "Serde: {}", error.to_string()),
            SizeTooHigh(size) => write!(
                f,
                "Size must be less than {}; got: {}",
                MAX_PS_FETCH_SIZE, size
            ),
        }
    }
}

impl Error for PSError {}

impl From<CSVError> for PSError {
    fn from(error: CSVError) -> Self {
        PSError::Csv(error)
    }
}

impl From<ParseError> for PSError {
    fn from(error: ParseError) -> Self {
        PSError::Parse(error)
    }
}

impl From<reqwest::Error> for PSError {
    fn from(error: reqwest::Error) -> Self {
        PSError::Reqwest(error)
    }
}

impl From<SerdeJSONError> for PSError {
    fn from(error: SerdeJSONError) -> Self {
        PSError::SerdeJson(error)
    }
}
