#[warn(clippy::all)]
use log::{debug, error, info};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Url;
use std::collections::HashSet;
use std::env::consts::OS;
use std::thread::sleep;
use std::time::Duration;

use super::nodestructs::{Node, PushshiftBase, RawNode};
use crate::nodecsv::nodecsv::{read_nodes, write_nodes};
use crate::pushshift::pserror::PSError;

#[derive(Debug)]
pub struct ScraperClient {
    client: Client,
    nodes: HashSet<Node>,
    urls: Vec<Url>, // Make into a HashMap to store UTC epoch
}

/// I designed ScraperClient specifically for my thesis, so I'm not sure if anyone else would
/// really use it for anything.
impl ScraperClient {
    pub fn new(timeout: u64, urls: &Vec<Url>) -> Result<Self, PSError> {
        Ok(ScraperClient {
            client: ScraperClient::make_client(timeout)?,
            urls: urls.clone(),
            nodes: HashSet::new(),
        })
    }

    fn make_client(timeout: u64) -> Result<Client, PSError> {
        Ok(ClientBuilder::new()
            .timeout(Duration::from_secs(timeout))
            .user_agent(format!(
                "<{platform}>:<{pkg}>:<{version}>",
                platform = OS,
                pkg = env!("CARGO_PKG_NAME"),
                version = env!("CARGO_PKG_VERSION")
            ))
            .build()?)
    }

    pub fn from_csv(timeout: u64, urls: &Vec<Url>, path: &str) -> Result<Self, PSError> {
        Ok(ScraperClient {
            client: ScraperClient::make_client(timeout)?,
            urls: urls.clone(),
            nodes: read_nodes(path)?,
        })
    }

    pub fn to_csv(&self, path: &str) -> Result<(), PSError> {
        Ok(write_nodes(path, &self.nodes)?)
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
