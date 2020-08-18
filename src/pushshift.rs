#[warn(clippy::all)]
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::{Error, Result};
use serde::Deserialize;

#[derive(Clone, Copy, Debug)]
enum PSEndpoint {
    Comment,
    Submission,
    Subreddit
}

impl PSEndpoint {
    fn as_str(&self) -> &'static str {
        match self {
            PSEndpoint::Comment => "/comment/search",
            PSEndpoint::Submission => "/submission/search",
            PSEndpoint::Subreddit => "/subreddit/search"
        }
    }
}

struct Pushshift {
    url: String,
    params: Vec<(String, String)>
}

impl Pushshift {
    fn new(endpoint: PSEndpoint) -> Self {
        Pushshift {
            url: "https://api.pushshift.io/reddit",
            params: Vec<String, String>::new()
        }
    }

    fn subreddit(&mut self, ) {
        unimplemented()!;
    }
}
