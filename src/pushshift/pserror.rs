#[warn(clippy::all)]
//use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub static MAX_PS_FETCH_SIZE: u32 = 1000;

#[derive(Debug, Clone)]
pub enum PSError {
    AlreadyAdded(String),
    InvalidSubreddit(String),
    NoParams,
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
            SizeTooHigh(size) => write!(
                f,
                "Size must be less than {}; got: {}",
                MAX_PS_FETCH_SIZE, size
            ),
        }
    }
}

impl Error for PSError {}
