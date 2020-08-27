#[warn(clippy::all)]
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Hash)]
pub enum TimeConvenience {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Weeks(u32),
    Years(u32),
    UTC(u64),
}

impl Display for TimeConvenience {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use TimeConvenience::*;
        match self {
            Seconds(time) => write!(f, "{}s", time),
            Minutes(time) => write!(f, "{}m", time),
            Hours(time) => write!(f, "{}h", time),
            Days(time) => write!(f, "{}d", time),
            Weeks(time) => write!(f, "{}w", time),
            Years(time) => write!(f, "{}y", time),
            UTC(time) => write!(f, "{}", time),
        }
    }
}
