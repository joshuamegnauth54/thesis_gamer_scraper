#[warn(clippy::all)]
mod pushshift;
mod scraperclient;

use pushshift::psendpoint::PSEndpoint;
use pushshift::pserror::PSError;
use pushshift::pushshiftbuilder::PushshiftBuilder;
use scraperclient::scraperclient::ScraperClient;

fn main() -> Result<(), PSError> {
    let ps4 = PushshiftBuilder::new(PSEndpoint::Subreddit)
        .subreddit("PS4")?
        .size(25)?
        .build()
        .unwrap();

    println!("{}", ps4);

    Ok(())
}
