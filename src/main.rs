// This code is from the Rustlang Nursery examples.
// I'm playing with reqwest/serde before I start the thesis program.
#[warn(clippy::all)]
use reqwest::blocking::{Client, ClientBuilder};
use serde::Deserialize;

mod pushshift;
mod scraperclient;

#[derive(Debug, Deserialize)]
struct User {
    login: String,
    id: u64,
}

fn stargazers(client: &Client, user: &str, repo: &str) -> Result<Vec<User>, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{user}/{repo}/stargazers",
        user = user,
        repo = repo
    );

    Ok(client.get(&url).send()?.json()?)
}

fn is_github_user(client: &Client, user: &str) -> Result<bool, reqwest::Error> {
    let url = format!("https://api.github.com/users/{user}", user = user);
    Ok(client.head(&url).send()?.status().is_success())
}

fn main() -> Result<(), reqwest::Error> {
    let user = "joshuamegnauth54";
    let repo = "KagglePractice";
    let timeout = std::time::Duration::new(10, 0);

    let client = ClientBuilder::new()
        .user_agent("Josh-Reqwest-Test")
        .timeout(timeout)
        .build()?;

    let users = stargazers(&client, user, repo)?;
    println!("{:?}", users);

    println!("Is Josh a user? {}", is_github_user(&client, user)?);

    Ok(())
}
