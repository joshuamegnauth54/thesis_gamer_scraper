use reqwest::blocking::{Client, ClientBuilder};
use reqwest::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;
#[warn(clippy::all)]
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum PSEndpoint {
    Comment,
    Submission,
    Subreddit,
}

// Free ToString!!
impl Display for PSEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
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

// Builds up a Pushshift object similar to how reqwest builds up Clients/Requests.
pub struct PushshiftBuilder {
    url: String,
    params: HashMap<String, Vec<String>>,
}

impl PushshiftBuilder {
    pub fn new(endpoint: PSEndpoint) -> Self {
        PushshiftBuilder {
            url: "https://api.pushshift.io/reddit".to_string() + &endpoint.to_string(),
            params: HashMap::new(),
        }
    }

    fn subreddit(&mut self) {
        unimplemented!();
    }

    fn add_param(&mut self, param: &str, par_options: &[String]) {
        unimplemented!();
    }
}
