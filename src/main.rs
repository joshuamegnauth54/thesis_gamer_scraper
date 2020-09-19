#[warn(clippy::all)]
use std::env;

mod nodecsv;
mod pushshift;
mod scraperclient;

use log::{error, info};
use pushshift::{
    psendpoint::PSEndpoint,
    pserror::{PSError, MAX_PS_FETCH_SIZE},
    pushshiftbuilder::PushshiftBuilder,
};
use scraperclient::scraperclient::ScraperClient;

static DEFAULT_SCRAPE: usize = 200000;
static DEFAULT_TIMEOUT: u64 = 90;

fn log_init() {
    let _log = pretty_env_logger::try_init_timed().map_err(|error| {
        eprintln!(
            "NOTE: Failed to initialize logger. Logging may be disabled. Error: {}",
            error
        )
    });
}

fn main() -> Result<(), PSError> {
    log_init();
    let subs: Vec<String> = env::args().skip(1).collect();
    if subs.len() == 0 {
        error!("No arguments supplied.");
        Err(PSError::NoArguments)?
    }

    let subreddit_urls = PushshiftBuilder::new(PSEndpoint::Comment)
        .size(MAX_PS_FETCH_SIZE)?
        .build_multiple(&subs)?;

    info!("Beginning scrape.");
    let mut scraper = ScraperClient::new(DEFAULT_TIMEOUT, &subreddit_urls)?;
    scraper.scrape_until(DEFAULT_SCRAPE)?;
    assert!(scraper.view_nodes().len() > 0);
    // scraper.scrape_individ_users()?;
    info!("Subreddits list: {:?}", subs);
    info!("Nodes scraped: {}", scraper.length_nodes());
    Ok(scraper.to_csv("/home/joshua/Documents/test.csv")?)
}
