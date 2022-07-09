#![feature(once_cell)]
use std::env;

pub mod nodecsv;
pub mod pushshift;
pub mod scraperclient;

use log::{error, info};
use pushshift::{PSEndpoint, PSError, PushshiftBuilder, MAX_PS_FETCH_SIZE};
use scraperclient::client::ScraperClient;

static DEFAULT_SCRAPE: usize = 125000;
static DEFAULT_TIMEOUT: u64 = 90;

// I'll have to add argument handling for paths later...
static DEFAULT_PATH: &str = "/home/joshua/Documents/";
static DEFAULT_NAME: &str = "gamer_ps.csv";

fn log_init() {
    let _log = pretty_env_logger::try_init_timed().map_err(|error| {
        eprintln!(
            "NOTE: Failed to initialize logger. Logging may be disabled. Error: {}",
            error
        )
    });
}

#[tokio::main]
async fn main() -> Result<(), PSError> {
    log_init();
    let subs: Vec<String> = env::args().skip(1).collect();
    if subs.is_empty() {
        error!("No arguments supplied.");
        Err(PSError::NoArguments)?
    }

    let subreddit_urls = PushshiftBuilder::new(PSEndpoint::Comment)
        .size(MAX_PS_FETCH_SIZE)?
        .build_multiple(&subs)?;

    info!("Beginning scrape.");
    let mut scraper = ScraperClient::new(DEFAULT_TIMEOUT, &subreddit_urls)?;
    scraper.scrape_until(DEFAULT_SCRAPE).await?;
    assert!(!scraper.view_nodes().is_empty());
    // scraper.scrape_individ_users()?;
    info!("Subreddits list: {:?}", subs);
    info!("Nodes scraped: {}", scraper.length_nodes());
    info!("Hashing names for privacy.");
    scraper.hash_names();
    scraper.to_csv(&format!("{}{}", DEFAULT_PATH, DEFAULT_NAME))
}
