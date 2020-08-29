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
            Sort::Desc => write!(f, "desc"),
            Sort::Asc => write!(f, "asc"),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Parameter::CreatedUTC => write!(f, "created_utc"),
            Parameter::Score => write!(f, "score"),
        }
    }
}
