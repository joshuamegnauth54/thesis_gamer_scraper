#[warn(clippy::all)]
use std::env;

mod nodecsv;
mod pushshift;
mod scraperclient;

use log::error;
use pushshift::psendpoint::PSEndpoint;
use pushshift::pserror::PSError;
use pushshift::pushshiftbuilder::PushshiftBuilder;
use scraperclient::scraperclient::ScraperClient;

static DEFAULT_SCRAPE: usize = 500;
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
        .subreddit("PS4")?
        .size(500)?
        .build_multiple(&subs)?;

    let mut scraper = ScraperClient::new(DEFAULT_TIMEOUT, &subreddit_urls)?;
    println!("{:?}", subs);
    scraper.scrape_until(DEFAULT_SCRAPE)?;
    assert!(scraper.view_nodes().len() > 0);
    scraper.scrape_individ_users()?;
    Ok(scraper.to_csv("/home/joshua/Documents/test.csv")?)
}
