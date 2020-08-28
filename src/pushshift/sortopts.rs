#[warn(clippy::all)]
use std::fmt::{Display, Formatter, Result};

pub enum Sort {
    Desc,
    Asc,
}

pub enum Parameter {
    CreatedUTC,
    Score,
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Desc => write!(f, "desc"),
            Asc => write!(f, "asc"),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CreatedUTC => write!(f, "created_utc"),
            Score => write!(f, "score"),
        }
    }
}
