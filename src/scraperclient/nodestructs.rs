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
// in our SerDe struct. Also, I'm not sure if this applies to every endpoint for PushShift.
#[derive(Debug, Deserialize)]
pub struct PushshiftBase {
    pub data: Vec<RawNode>,
}

/// RawNode is a Node plus some associated metadata such as the time or comment rating.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RawNode {
    /// Redditor who wrote the post
    author: String,
    body: String,
    /// Creation time for the post
    pub created_utc: u64,
    /// Link to post.
    permalink: String,
    /// Post rating
    score: i32,
    /// Subreddit for post
    subreddit: String,
}

// All of the members are public because of the scraper.
/// Nodes are parsed RawNodes with only the data required for my thesis.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Node {
    // Vertex
    pub author: String,
    // Maybe to add weights by posts?
    pub created_utc: u64,
    // Alternate edge
    pub permalink: String,
    // Main edge
    pub subreddit: String,
}

// Parse relevant information from raw nodes
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
