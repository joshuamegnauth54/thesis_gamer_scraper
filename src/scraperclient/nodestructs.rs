#[warn(clippy::all)]
use serde::{Deserialize, Serialize};
use std::convert::From;

// Root data type for scraping from https://reddit.com/{user}.json
#[derive(Debug, Deserialize)]
pub struct RedditUserRoot {
    pub data: RedditUserBase,
}

#[derive(Debug, Deserialize)]
pub struct RedditUserBase {
    pub children: Vec<PushshiftBase>,
}

// The root data type returned by PushShift is an array so we have to store the "data" field first
// in our SerDe struct. Also, I'm not sure if this applies to every end point for PushShift.
#[derive(Debug, Deserialize)]
pub struct PushshiftBase {
    pub data: Vec<RawNode>,
}

// RawNode is our Node plus some associated metadata such as the time.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RawNode {
    author: String,
    body: String,
    pub created_utc: u64,
    permalink: String,
    score: i32,
    subreddit: String,
}

// Node contains only the data I need.
// All of the members are public because of the scraper.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Node {
    pub author: String,    // Vertex
    pub created_utc: u64,  // Maybe to add weights by posts?
    pub permalink: String, // Alternate edge
    pub subreddit: String, // Main edge
}

impl From<RawNode> for Node {
    fn from(raw: RawNode) -> Self {
        Node {
            author: raw.author,
            created_utc: raw.created_utc,
            permalink: raw.permalink,
            subreddit: raw.subreddit,
        }
    }
}

impl From<&RawNode> for Node {
    fn from(raw: &RawNode) -> Self {
        Node {
            author: raw.author.clone(),
            created_utc: raw.created_utc,
            permalink: raw.permalink.clone(),
            subreddit: raw.subreddit.clone(),
        }
    }
}

impl From<RedditUserRoot> for RedditUserBase {
    fn from(userroot: RedditUserRoot) -> Self {
        RedditUserBase {
            children: userroot.data.children,
        }
    }
}
