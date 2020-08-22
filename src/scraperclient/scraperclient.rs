#[warn(clippy::all)]
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Url;
use serde::Deserialize;
use std::env::consts::OS;

struct ScraperClient {
    client: Client,
    urls: Vec<Url>,
}

impl ScraperClient {
    fn new(timeout: u64) -> Self {
        ScraperClient {
            client: ClientBuilder::new()
                .timeout(std::time::Duration::new(timeout, 0))
                .user_agent(format!(
                    "<{platform}>:<{pkg}>:<{version}>",
                    platform = OS,
                    pkg = env!("CARGO_PKG_NAME"),
                    version = env!("CARGO_PKG_VERSION")
                )),
        }
    }
}
