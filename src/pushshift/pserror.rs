#[warn(clippy::all)]
use csv::Error as CSVError;
use serde_json::Error as SerdeJSONError;
use std::{
    convert::From,
    error::Error,
    fmt::{Display, Formatter},
    io::Error as IoError,
};
use url::ParseError;

pub static MAX_PS_FETCH_SIZE: u32 = 1000;

#[derive(Debug)]
pub enum PSError {
    AlreadyAdded(String),
    Csv(CSVError),
    Io(IoError),
    InvalidSubreddit(String),
    NoMoreNodes,
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
            Io(error) => write!(f, "IO: {}", error.to_string()),
            InvalidSubreddit(subreddit) => write!(
                f,
                "Subreddit may only contain alphanumeric and _: {}",
                subreddit
            ),
            NoMoreNodes => write!(f, "No more nodes to scrape."),
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

impl From<IoError> for PSError {
    fn from(error: IoError) -> Self {
        PSError::Io(error)
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
