use std::fmt::{Display, Error, Formatter};

/// PushShift API endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PSEndpoint {
    Comment,
    Submission,
    Subreddit,
}

impl Display for PSEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                PSEndpoint::Comment => "/comment/search",
                PSEndpoint::Submission => "/submission/search",
                PSEndpoint::Subreddit => "/subreddit/search",
            }
        )
    }
}
