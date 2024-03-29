use futures::TryFutureExt;
use log::{debug, error, info};
use reqwest::{Client, ClientBuilder, Url};
use ring::digest::{Context, Digest, SHA256};
use std::{collections::HashSet, env::consts::OS, path::Path, sync::LazyLock, time::Duration};
use tokio::time::sleep;

use super::nodestructs::{Node, PushshiftBase, RawNode};
use crate::{
    nodecsv::nodeio::{read_nodes, write_nodes},
    pushshift::PSError,
};

static DEFAULT_BACKOFF: u64 = 10;
static DEFAULT_THRESH: u8 = 3;
static TOPIC_POS: usize = 5;
// Invalid or unusable scraped data.
static NOT_USERS: LazyLock<Vec<String>> =
    LazyLock::new(|| vec!["[deleted]".to_string(), "AutoModerator".to_string()]);

#[derive(Debug)]
pub struct ScraperClient {
    backoff_time: u64,
    client: Client,
    nodes: HashSet<Node>,
    urls: Vec<Url>,
    zero_length_scrapes: u8,
}

/// I designed ScraperClient specifically for my thesis, so I'm not sure if anyone else would
/// really use it for anything. Much of the code is patchwork and messy, but I've learned a lot
/// during implementation.
impl ScraperClient {
    pub fn new(timeout: u64, urls: &[Url]) -> Result<Self, PSError> {
        Ok(ScraperClient {
            backoff_time: DEFAULT_BACKOFF,
            client: ScraperClient::make_client(timeout)?,
            nodes: HashSet::new(),
            urls: urls.to_vec(),
            zero_length_scrapes: 0,
        })
    }

    // The following user agent is more or less the recommended agent.
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

    pub fn from_csv<P>(timeout: u64, urls: &[Url], path: P) -> Result<Self, PSError>
    where
        P: AsRef<Path>,
    {
        Ok(ScraperClient {
            backoff_time: DEFAULT_BACKOFF,
            client: ScraperClient::make_client(timeout)?,
            urls: urls.to_vec(),
            nodes: read_nodes(path)?,
            zero_length_scrapes: 0,
        })
    }

    pub fn to_csv<P>(&self, path: P) -> Result<(), PSError>
    where
        P: AsRef<Path>,
    {
        write_nodes(path, &self.nodes)
    }

    pub fn length_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn view_nodes(&self) -> &HashSet<Node> {
        &self.nodes
    }

    // Encode bytes with SHA256 using Ring
    fn sha256(data: &[u8]) -> Digest {
        let mut context = Context::new(&SHA256);
        context.update(data);
        context.finish()
    }

    /// Hashes names of posters/topics.
    /// Topics are extracted from the _permalink_ field.
    pub fn hash_names(&mut self) {
        let mut hashed_names = self
            .nodes
            .drain()
            .map(|old_node| Node {
                author: hex::encode(Self::sha256(old_node.author.as_bytes())),
                created_utc: old_node.created_utc,
                permalink: old_node
                    .permalink
                    .split('/')
                    .nth(TOPIC_POS)
                    .map_or(String::from("NA"), |topic| {
                        hex::encode(Self::sha256(topic.as_bytes()))
                    }),
                subreddit: old_node.subreddit,
            })
            .collect();

        std::mem::swap(&mut self.nodes, &mut hashed_names);
    }

    /// Snowball samples edges by using each unique username to gather a list of subreddits to
    /// which they post.
    pub async fn scrape_individ_users(&mut self) -> Result<(), PSError> {
        // We need to collect the usernames as Strings into a HashSet first
        // to filter out duplicates. The actual HashSet contains duplicate usernames but unique
        // nodes due to the epoch timestamp of each post.
        let users: HashSet<_> = self
            .nodes
            .iter()
            .map(|node| format!("https://reddit.com/user/{}.json", node.author,))
            .collect();
        // Next collect the actual Nodes.
        info!("Scraping individual users.");
        let mut users_deser: Vec<Node> = Vec::new();
        for user_url in users.iter() {
            users_deser.extend(self.client.get(user_url).send().await?.json().await);
            self.backoff().await;
        }
        self.nodes.extend(users_deser.into_iter());

        Ok(())
    }

    /// Scrapes until node_limit is reached.
    pub async fn scrape_until(&mut self, node_limit: usize) -> Result<(), PSError> {
        while self.length_nodes() < node_limit {
            info!("Node length: {}", self.length_nodes());
            if self.scrape_nodes().await? == 0 {
                // This whole paradigm is ugly. I need to clean it up.
                if self.zero_length_scrapes == DEFAULT_THRESH {
                    return Err(PSError::NoMoreNodes);
                }
                self.exponential_backoff();
            }
            self.zero_length_scrapes = 0
        }
        debug!(
            "scrape_until ended with {}/{}.",
            self.length_nodes(),
            node_limit
        );
        Ok(())
    }

    // Non-accounts such as deleted posts/users are scraped as well.
    fn filter_junk(&mut self) {
        // I have to filter here because I'm comparing the usernames which are NOT nodes.
        // Therefore, I can't use set logic.
        let bad_nodes: Vec<_> = self
            .nodes
            .iter()
            .filter(|node| NOT_USERS.iter().any(|nonuser| *nonuser == node.author))
            .cloned()
            .collect();

        for bad_node in bad_nodes {
            debug!("Bad node: {:?}", bad_node);
            self.nodes.remove(&bad_node);
        }
    }

    // Sleeps before resuming a scrape.
    async fn backoff(&self) {
        info!("Sleeping: {} seconds", self.backoff_time);
        sleep(Duration::from_secs(self.backoff_time)).await;
    }

    fn exponential_backoff(&mut self) {
        // Increase the backoff timer by timer^2 up to a maximum of 60
        self.backoff_time = std::cmp::max(self.backoff_time.pow(2), 60);
    }

    fn replace_before(url: &str, epoch: u64) -> Result<Url, PSError> {
        debug!("Replacing parameter before in: {}", url);
        // First we need to turn the &str to a Url. This shouldn't fail since the string slice
        // directly comes from an already parsed URL. However, I check anyway because I'm not sure
        // what the paradigm is here yet.
        let new_url = Url::parse(url).map_err(|error| {
            error!(
                "Failed to parse URL: {} while paginating. Error: {}",
                url, error
            );
            error
        })?;
        // Next, the "before" query must be filtered out of the URL followed by pushing the new
        // query into the Vector.
        // I can't figure out how to change the actual query without going through this lengthy
        // filtering process.
        let mut qpairs: Vec<_> = new_url
            .query_pairs()
            .into_owned()
            .filter(|query_pair| query_pair.0 != "before")
            .collect();
        qpairs.push((String::from("before"), epoch.to_string()));

        // If the URL doesn't contain a host then something is hopelessly wrong
        Ok(Url::parse_with_params(
            &(String::from("https://")
                + new_url
                    .host_str()
                    .ok_or(PSError::Parse(url::ParseError::EmptyHost))?
                + new_url.path()),
            qpairs,
        )?)
    }

    // I'll refactor this after gathering my thesis data.
    // Essentially performs a convenience sample.
    pub async fn scrape_nodes(&mut self) -> Result<usize, PSError> {
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
                .await
                .map_err(PSError::Reqwest);

            match result {
                Ok(scraped) => {
                    for val in scraped.data.iter() {
                        debug!("{:?}", val);
                    }
                    info!("Scraped {} nodes from {}.", scraped.data.len(), url_str);
                    if !scraped.data.is_empty() {
                        new_urls.push(ScraperClient::replace_before(
                            url_str,
                            scraped
                                .data
                                .iter()
                                // Find the lowest date-time stamp for the new "before" parameter.
                                .min_by(|x, y| x.created_utc.cmp(&y.created_utc))
                                .unwrap() // Okay to unwrap because we're comparing two u64
                                .created_utc, // ...and finally select the actual stamp
                        )?);
                        nodes.extend(scraped.data.into_iter());
                    } else {
                        // We shouldn't raise an error here because we may have more URLs to check.
                        // Zero nodes may not be an error for a particular URL.
                        info!("No more nodes in: {}", url_str);
                    }
                }
                // Any actual errors are reported, but we continue scraping instead of failing to
                // be safe.
                Err(error) => error!("{} @ {}", error, url_str),
            }
            self.backoff().await;
        }
        // Replace the old URLs with the new URLs with the new "before" query pairs.
        // This is messy and I don't like it. :(
        self.urls.clear();
        self.urls.extend(new_urls.into_iter());
        // Add newly scraped nodes to our main list of nodes and remove junk nodes.
        self.nodes.extend(nodes.iter().map(|node| node.into()));
        self.filter_junk();
        Ok(nodes.len())
    }
}
