#[warn(clippy::all)]

pub enum PSError {
    AlreadyAdded(String),
    InvalidSubreddit(String),
}
