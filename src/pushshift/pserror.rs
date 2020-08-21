use std::convert::From;
use std::error::Error;
#[warn(clippy::all)]
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum PSError {
    AlreadyAdded(String),
    InvalidSubreddit(String),
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
        }
    }
}
