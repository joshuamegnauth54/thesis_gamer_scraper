#[warn(clippy::all)]
mod pushshift;
mod scraperclient;

use pushshift::psendpoint::PSEndpoint;
use pushshift::pserror::PSError;
use pushshift::pushshiftbuilder::PushshiftBuilder;
use scraperclient::scraperclient::Node;
use scraperclient::scraperclient::ScraperClient;

fn main() -> Result<(), PSError> {
    let _log = pretty_env_logger::try_init()
        .map_err(|error| eprintln!("Couldn't initialized logger: {}", error));

    let subs = PushshiftBuilder::new(PSEndpoint::Comment)
        .subreddit("PS4")?
        .size(25)?
        .build_multiple(&["PS4", "pcgaming", "pcmasterrace", "PS3"])?;

    let scraper = ScraperClient::new(90, &subs).unwrap();
    println!("{:?}", subs);
    let nodes = scraper.scrape_nodes();
    println!("{:?}", nodes);
    scraper.remove_me();

    Ok(())
}
