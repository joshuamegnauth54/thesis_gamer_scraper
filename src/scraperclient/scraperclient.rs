#[warn(clippy::all)]
use log::{debug, error, info};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashSet;
use std::convert::From;
use std::env::consts::OS;
use std::thread::sleep;
use std::time::Duration;

use crate::pushshift::pserror::PSError;

// The root data type returned by PushShift is an array so we have to store the "data" field first
// in our SerDe struct. Also, I'm not sure if this applies to every end point for PushShift.
#[derive(Debug, Deserialize)]
struct PushshiftBase {
    data: Vec<RawNode>,
}

// RawNode is our Node plus some associated metadata such as the time.
#[derive(Clone, Hash, Eq, PartialEq, Deserialize, Debug)]
pub struct RawNode {
    author: String,
    body: String,
    created_utc: u64,
    score: u32,
    subreddit: String,
}

// Node contains only the data I need.
#[derive(Clone, Hash, Eq, PartialEq, Deserialize, Debug)]
pub struct Node {
    author: String,    // Vertex
    created_utc: u64,  // Maybe to add weights by posts?
    subreddit: String, // Edge
}

impl From<RawNode> for Node {
    fn from(raw: RawNode) -> Self {
        Node {
            author: raw.author,
            created_utc: raw.created_utc,
            subreddit: raw.subreddit,
        }
    }
}

impl From<&RawNode> for Node {
    fn from(raw: &RawNode) -> Self {
        Node {
            author: raw.author.clone(),
            created_utc: raw.created_utc,
            subreddit: raw.subreddit.clone(),
        }
    }
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

    pub fn view_nodes(&self) -> &HashSet<Node> {
        &self.nodes
    }

    pub fn scrape_nodes(&mut self) {
        let mut nodes: HashSet<RawNode> = HashSet::new();
        for url in self.urls.iter().map(|url| url.as_str()) {
            let result: Result<PushshiftBase, PSError> = self
                .client
                .get(url)
                .send()
                .and_then(|response| response.json())
                .map_err(|error| PSError::Reqwest(error));

            match result {
                Ok(scraped) => {
                    for val in scraped.data.iter() {
                        debug!("{:?}", val);
                    }
                    info!("Scraped {} nodes from {}.", scraped.data.len(), url);
                    nodes.extend(scraped.data.into_iter())
                }
                Err(error) => error!("{} @ {}", error, url),
            }
            info!("Sleeping for two seconds.");
            sleep(Duration::from_secs(2));
        }
        self.nodes.extend(nodes.iter().map(|node| node.into()));
    }
}
