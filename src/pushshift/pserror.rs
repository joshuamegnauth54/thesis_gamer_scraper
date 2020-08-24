#[warn(clippy::all)]
use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};
use url::ParseError;

pub static MAX_PS_FETCH_SIZE: u32 = 1000;

#[derive(Debug, Clone)]
pub enum PSError {
    AlreadyAdded(String),
    InvalidSubreddit(String),
    NoParams,
    Parse(ParseError),
    SizeTooHigh(u32),
}

impl Display for PSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PSError::*;
        match self {
            AlreadyAdded(param) => write!(f, "Parameter already added: {}", param),
            InvalidSubreddit(subreddit) => write!(
                f,
                "Subreddit may only contain alphanumeric and _: {}",
                subreddit
            ),
            NoParams => write!(
                f,
                "No parameters found. You have to specify parameters such as a subreddit."
            ),
            Parse(error) => write!(f, "{}", error.to_string()),
            SizeTooHigh(size) => write!(
                f,
                "Size must be less than {}; got: {}",
                MAX_PS_FETCH_SIZE, size
            ),
        }
    }
}

impl Error for PSError {}

impl From<ParseError> for PSError {
    fn from(error: ParseError) -> Self {
        PSError::Parse(error)
    }
}
