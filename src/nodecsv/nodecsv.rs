#[warn(clippy::all)]
use csv::{Reader, Writer};
use lazy_static::lazy_static;
use log::error;
use std::collections::HashSet;

use crate::pushshift::pserror::PSError;
use crate::scraperclient::nodestructs::Node;

/// Reads all Nodes from a CSV file into a HashSet.
/// Fails if the file cannot be read but reports errors while reading instead of failing.
pub fn read_nodes(path: &str) -> Result<HashSet<Node>, PSError> {
    lazy_static! {
        // Had to cheat to get past the borrow checker by cloning the Position object.
        // Position::set_record returns a &mut which we can't use in lazy_static (I think)
        static ref POSITION: csv::Position = csv::Position::new().set_record(u64::MAX).clone();
    }

    let split_results: (Vec<_>, Vec<_>) = Reader::from_path(path)?
        .deserialize()
        .partition(|result| result.is_ok());

    split_results.1.into_iter().for_each(|wrapped_error| {
        wrapped_error.err().map(|result| {
            error!(
                "Failed to parse row: {row}. Reason: {error}",
                row = result.position().unwrap_or(&POSITION).record(),
                error = result
            )
        });
    });

    Ok(split_results
        .0
        .into_iter()
        .map(|result| result.unwrap())
        .collect())
}

pub fn write_nodes(path: &str, nodes: &HashSet<Node>) -> Result<(), PSError> {
    let mut writer = Writer::from_path(path)?;
    for node in nodes.iter() {
        writer
            .serialize(node)
            .err()
            .map(|error| error!("Failed to write a row: {}", error.to_string()));
    }

    Ok(writer.flush()?)
}
