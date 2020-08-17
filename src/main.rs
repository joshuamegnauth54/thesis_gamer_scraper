use reqwest::blocking::ClientBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    login: String,
    id: u64,
}

fn main() -> Result<(), reqwest::Error> {
    let user = "joshuamegnauth54";
    let repo = "KagglePractice";
    let url = format!(
        "https://api.github.com/repos/{user}/{repo}/stargazers",
        user = user,
        repo = repo
    );
    println!("{}", url);

    let client = ClientBuilder::new()
        .user_agent("Josh-Reqwest-Test")
        .build()?;

    let response = client.get(&url).send()?;
    let repjson: Vec<User> = response.json()?;

    println!("{:?}", repjson);

    Ok(())
}
