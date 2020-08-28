#[warn(clippy::all)]
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::pushshift::pserror::PSError;
use crate::scraperclient::nodestructs::Node;

pub fn read_nodes(path: &str) -> Result<HashSet<Node>, PSError> {
    let split_results: (Vec<_>, Vec<_>) = Reader::from_path(path)?
        .deserialize()
        .partition(|result| result.is_ok());

    for error in split_results.1 {}

    Ok(split_results
        .0
        .iter()
        .map(|result| result.unwrap())
        .collect())
}

pub fn write_nodes(path: &str, nodes: &HashSet<Node>) -> Result<(), PSError> {
    Writer::from_path(path)?
}
