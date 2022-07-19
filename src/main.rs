#![feature(once_cell)]
pub mod nodecsv;
pub mod pushshift;
pub mod scraperclient;

use clap::Parser;
use log::{error, info};
use pushshift::{PSEndpoint, PSError, PushshiftBuilder, MAX_PS_FETCH_SIZE};
use scraperclient::client::ScraperClient;
use std::path::PathBuf;

// Default number of items to scrape
static DEFAULT_SCRAPE: usize = 125000;
static DEFAULT_TIMEOUT: u64 = 90;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct ScrapeOpts {
    /// Path to resume scrape from or where to save a new scrape.
    #[clap(required = true, value_parser)]
    path: PathBuf,
    /// Subreddits to scrape
    #[clap(required = true, value_parser)]
    subs: Vec<String>,
    /// Amount of nodes to scrape
    #[clap(default_value_t = DEFAULT_SCRAPE, short, long, value_parser)]
    amount: usize,
    /// Timeout to wait for each individual request
    #[clap(default_value_t = DEFAULT_TIMEOUT, short, long, value_parser)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), PSError> {
    if pretty_env_logger::try_init_timed().is_err() {
        eprintln!("Failed to initialize logger. Logging may be disabled.")
    };

    // Argument handling
    let arguments = ScrapeOpts::parse();
    if arguments.subs.is_empty() {
        error!("No subreddits supplied.");
        Err(PSError::NoArguments)?
    }

    // Build scrapers
    let subreddit_urls = PushshiftBuilder::new(PSEndpoint::Comment)
        .size(MAX_PS_FETCH_SIZE)?
        .build_multiple(&arguments.subs)?;

    info!("Subreddits list: {:#?}", arguments.subs);
    let mut scraper = if arguments.path.exists() && arguments.path.is_file() {
        info!("Resuming scrape at {}", &arguments.path.to_string_lossy());
        ScraperClient::from_csv(arguments.timeout, &subreddit_urls, &arguments.path)?
    } else {
        info!("Beginning new scrape.");
        ScraperClient::new(arguments.timeout, &subreddit_urls)?
    };

    info!("Scraping until {} nodes", arguments.amount);
    scraper.scrape_until(arguments.amount).await?;
    assert!(!scraper.view_nodes().is_empty());
    // scraper.scrape_individ_users()?;
    info!("Nodes scraped: {}", scraper.length_nodes());
    info!("Hashing names for privacy.");
    scraper.hash_names();
    info!("Writing to CSV.");
    scraper.to_csv(&arguments.path)
}
