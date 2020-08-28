#[warn(clippy::all)]
mod nodecsv;
mod pushshift;
mod scraperclient;

use pushshift::psendpoint::PSEndpoint;
use pushshift::pserror::PSError;
use pushshift::pushshiftbuilder::PushshiftBuilder;
use pushshift::sortopts::{Parameter, Sort};
use scraperclient::nodestructs::Node;
use scraperclient::scraperclient::ScraperClient;

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
    let subs = PushshiftBuilder::new(PSEndpoint::Comment)
        .subreddit("PS4")?
        .size(500)?
        .sort(Sort::Desc, Parameter::CreatedUTC)?
        .build_multiple(&["PS4", "pcgaming", "pcmasterrace", "PS3"])?;

    let mut scraper = ScraperClient::new(90, &subs)?;
    println!("{:?}", subs);
    scraper.scrape_nodes();
    assert!(scraper.view_nodes().len() > 0);
    Ok(())
}
