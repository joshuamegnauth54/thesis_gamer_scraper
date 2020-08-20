#[warn(clippy::all)]
use reqwest::blocking::{Client, ClientBuilder};
use serde::Deserialize;
use std::collections::HashMap;

use crate::psendpoint::PSEndpoint;
use crate::pserror::PSError;

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

    fn subreddit(&mut self, sub: &str) -> Result<(), PSError> {}

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
