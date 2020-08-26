#[warn(clippy::all)]
use log::{error, info};
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

    pub fn remove_me(&self) {
        let texts: Result<Vec<String>, reqwest::Error> = self
            .urls
            .iter()
            .map(|url| url.as_str())
            .map(|url| {
                self.client
                    .get(url)
                    .send()
                    .and_then(|response| response.text())
            })
            .collect();

        let parsed: Vec<serde_json::Value> = texts
            .unwrap()
            .iter()
            .map(|text| serde_json::from_str(&text))
            .collect::<Result<Vec<serde_json::Value>, serde_json::Error>>()
            .unwrap();

        for json in parsed.iter() {
            println!(
                "{}",
                serde_json::to_string_pretty(json).unwrap_or("ERROR GOT DANG IT".to_owned())
            );
        }
    }

    pub fn scrape_nodes(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        for url in self.urls.iter().map(|url| url.as_str()) {
            let result: Result<Vec<Node>, PSError> = self
                .client
                .get(url)
                .send()
                .and_then(|response| response.json())
                .map_err(|error| PSError::Reqwest(error));

            match result {
                Ok(mut scraped) => {
                    info!("Scraped {} nodes from {}.", scraped.len(), url);
                    nodes.append(&mut scraped)
                }
                Err(error) => error!("{} @ {}", error, url),
            }
        }
        nodes
    }
}
