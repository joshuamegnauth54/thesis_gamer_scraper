#[warn(clippy::all)]
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::{Client, ClientBuilder};
use serde::Deserialize;
use std::collections::HashMap;

use super::psendpoint::PSEndpoint;
use super::pserror::PSError;

// Builds up a Pushshift object similar to how reqwest builds up Clients/Requests.
pub struct PushshiftBuilder {
    url: String,
    params: HashMap<String, String>,
}

impl PushshiftBuilder {
    pub fn new(endpoint: PSEndpoint) -> Self {
        PushshiftBuilder {
            url: "https://api.pushshift.io/reddit".to_string() + &endpoint.to_string(),
            params: HashMap::new(),
        }
    }

    fn subreddit(&mut self, sub: &str) -> Result<(), PSError> {
        lazy_static! {
            static ref INVALID_REDDIT: Regex = Regex::new(r"[^\w\d_]+").unwrap();
        }
        if INVALID_REDDIT.is_match(sub) {
            Err(PSError::InvalidSubreddit(sub.to_string()))
        } else {
            self.add_param("subreddit", sub)?
        }
    }

    fn add_param(&mut self, param: &str, par_options: &str) -> Result<(), PSError> {
        if self.params.contains_key(param) {
            Err(PSError::AlreadyAdded(param.to_string()))
        } else {
            self.params
                .insert(param.to_string(), par_options.to_string());
            Ok(())
        }
    }
}
