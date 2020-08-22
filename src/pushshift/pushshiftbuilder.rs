#[warn(clippy::all)]
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;
use url::ParseError;

use super::psendpoint::PSEndpoint;
use super::pserror::PSError;
use super::pserror::MAX_PS_FETCH_SIZE;

// Builds up a Pushshift object similar to how reqwest builds up Clients/Requests.
#[derive(Clone, Debug)]
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

    pub fn build(self) -> Result<Url, ParseError> {
        unimplemented!();
    }

    pub fn clone_replace_sub(&self, sub: &str) -> Result<PushshiftBuilder, PSError> {
        let mut newself = self.clone();
        let _ignore = newself.params.remove("subreddit");
        newself.subreddit(sub)?;
        Ok(newself)
    }

    pub fn subreddit(&mut self, sub: &str) -> Result<(), PSError> {
        // Using the lazy_static! macro is good practice according to the regex crate docs.
        lazy_static! {
            // I tested the RegEx below so we may unwrap() safely.
            static ref INVALID_REDDIT: Regex = Regex::new(r"[^\w\d_]+").unwrap();
        }
        if INVALID_REDDIT.is_match(sub) {
            Err(PSError::InvalidSubreddit(sub.to_string()))
        } else {
            Ok(self.add_param("subreddit", sub)?)
        }
    }

    pub fn size(&mut self, size: u32) -> Result<(), PSError> {
        if size <= MAX_PS_FETCH_SIZE {
            Ok(self.add_param("size", &size.to_string())?)
        } else {
            Err(PSError::SizeTooHigh(size))
        }
    }

    fn add_param(&mut self, param: &str, par_options: &str) -> Result<(), PSError> {
        // Add_param() only allows a parameter to be added once. I decided against allowing
        // replacement in order to be as explicit as possible.
        if self.params.contains_key(param) {
            Err(PSError::AlreadyAdded(param.to_string()))
        } else {
            self.params
                .insert(param.to_string(), par_options.to_string());
            Ok(())
        }
    }
}
