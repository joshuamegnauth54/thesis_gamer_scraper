#[warn(clippy::all)]
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::{Error, Url};
use serde::Deserialize;
use std::collections::HashSet;
use std::env::consts::OS;

#[derive(Clone, Hash, Eq, PartialEq, Deserialize, Debug)]
struct Node {
    author: String,
    subreddit: String,
}

pub struct ScraperClient {
    client: Client,
    urls: Vec<Url>,
    nodes: HashSet<Node>,
}

impl ScraperClient {
    pub fn new(timeout: u64) -> Result<Self, Error> {
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
            urls: Vec::new(),
            nodes: HashSet::new(),
        })
    }
}
