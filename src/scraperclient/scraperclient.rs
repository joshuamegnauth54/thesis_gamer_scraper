#[warn(clippy::all)]
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashSet;
use std::env::consts::OS;

use crate::pushshift::pserror::PSError;

#[derive(Clone, Hash, Eq, PartialEq, Deserialize, Debug)]
pub struct Node {
    author: String,
    subreddit: String,
}

#[derive(Debug)]
pub struct ScraperClient {
    client: Client,
    nodes: HashSet<Node>,
    urls: Vec<Url>,
}

/// I designed ScraperClient specifically for my thesis, so I'm not sure if anyone else would
/// really use it for anything.
impl ScraperClient {
    pub fn new(timeout: u64, urls: &Vec<Url>) -> Result<Self, PSError> {
        Ok(ScraperClient {
            client: ClientBuilder::new()
                .timeout(std::time::Duration::new(timeout, 0))
                .user_agent(format!(
                    "<{platform}>:<{pkg}>:<{version}>",
                    platform = OS,
                    pkg = env!("CARGO_PKG_NAME"),
                    version = env!("CARGO_PKG_VERSION")
                ))
                .build()?,
            urls: urls.clone(),
            nodes: HashSet::new(),
        })
    }

    pub fn from_csv(path: &str) -> Result<Self, PSError> {
        unimplemented!()
    }

    pub fn to_csv(path: &str) -> Result<(), std::io::Error> {
        unimplemented!()
    }

    pub fn length_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn reqwest_error(error: reqwest::Error) -> reqwest::Error {
        error
    }

    fn json_error(error: reqwest::Error) -> reqwest::Error {
        error
    }

    // Not an actual test function.
    // Fix the ugly conversion from Url to &str somehow later
    // And maybe do a call back thingy to log errors or something?
    pub fn test(&mut self) -> Vec<Node> {
        let mut nodes_test: Vec<Node> = Vec::new();
        for url in self.urls.iter().map(|url| url.as_str()) {
            let results: Result<Node, reqwest::Error> = self
                .client
                .get(url)
                .send()
                .map_err(|error| ScraperClient::reqwest_error(error))
                .and_then(|response| response.json())
                .map_err(|error| ScraperClient::json_error(error));
            println!("{:?}", results);
        }
        nodes_test
    }
}
