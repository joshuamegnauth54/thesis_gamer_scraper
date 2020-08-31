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

    pub fn scrape_until(&mut self, node_limit: u32) {
        unimplemented!()
    }

    fn replace_before(url: &str, epoch: u64) -> Result<Url, PSError> {
        let new_url = Url::parse(url).map_err(|error| {
            error!(
                "Failed to parse URL: {} while paginating. Error: {}",
                url, error
            );
            error
        })?;
        let mut qpairs: Vec<_> = new_url
            .query_pairs()
            .into_owned()
            .filter(|query_pair| query_pair.0 != "before")
            .collect();
        qpairs.push((String::from("before"), epoch.to_string()));

        Ok(Url::parse("Shh")?)
    }

    pub fn scrape_nodes(&mut self) -> Result<(), PSError> {
        // Nodes holds RawNodes in case I decide to use the extra information
        // in any way.
        let mut nodes: HashSet<RawNode> = HashSet::new();
        let mut new_urls: Vec<Url> = Vec::with_capacity(self.urls.len());
        for url_str in self.urls.iter().map(|url| url.as_str()) {
            let result: Result<PushshiftBase, PSError> = self
                .client
                .get(url_str)
                .send()
                .and_then(|response| response.json())
                .map_err(|error| PSError::Reqwest(error));

            match result {
                Ok(scraped) => {
                    for val in scraped.data.iter() {
                        debug!("{:?}", val);
                    }
                    info!("Scraped {} nodes from {}.", scraped.data.len(), url_str);
                    if !scraped.data.is_empty() {
                        nodes.extend(scraped.data.into_iter());
                        new_urls.push(ScraperClient::replace_before(
                            &url_str,
                            scraped
                                .data
                                .iter()
                                .min_by(|x, y| x.created_utc.cmp(y.created_utc))
                                .unwrap() // Okay to unwrap because we're comparing two u64
                                .created_utc,
                        )?);
                    } else {
                        info!("No more nodes in: {}", url_str);
                    }
                }
                Err(error) => error!("{} @ {}", error, url_str),
            }
            info!("Sleeping for two seconds.");
            sleep(Duration::from_secs(2));
        }
        Ok(self.nodes.extend(nodes.iter().map(|node| node.into())))
    }
}
