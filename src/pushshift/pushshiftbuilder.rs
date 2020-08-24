#[warn(clippy::all)]
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;
use std::collections::HashMap;
use url::ParseError;

use super::psendpoint::PSEndpoint;
use super::pserror::PSError;
use super::pserror::MAX_PS_FETCH_SIZE;

/// Builds a reqwest::Url for the PushShift Reddit API.
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

    /// Builds the PushShift API call provided that the caller specified some parameters.
    /// This function doesn't consume self to facilitate building new URLs using _replace_sub_.
    pub fn build(&self) -> Result<Url, PSError> {
        if !self.params.is_empty() {
            Ok(Url::parse_with_params(&self.url, &self.params)?)
        } else {
            Err(PSError::NoParams)
        }
    }

    pub fn build_multiple(&mut self, subs: &[&str]) -> Vec<Result<Url, PSError>> {
        subs.iter()
            .map(|sub| self.replace_sub(sub)?.build())
            .collect()
    }

    pub fn replace_sub(&mut self, sub: &str) -> Result<&mut Self, PSError> {
        let _ignore = self.params.remove("subreddit");
        Ok(self.subreddit(sub)?)
    }

    pub fn subreddit(&mut self, sub: &str) -> Result<&mut Self, PSError> {
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

    pub fn size(&mut self, size: u32) -> Result<&mut Self, PSError> {
        if size <= MAX_PS_FETCH_SIZE {
            Ok(self.add_param("size", &size.to_string())?)
        } else {
            Err(PSError::SizeTooHigh(size))
        }
    }

    fn add_param(&mut self, param: &str, par_options: &str) -> Result<&mut Self, PSError> {
        // Add_param() only allows a parameter to be added once. I decided against allowing
        // replacement in order to be as explicit as possible.
        if self.params.contains_key(param) {
            Err(PSError::AlreadyAdded(param.to_string()))
        } else {
            self.params
                .insert(param.to_string(), par_options.to_string());
            Ok(self)
        }
    }
}
