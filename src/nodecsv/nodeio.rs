use csv::{Reader, Writer};
use log::error;
use std::{collections::HashSet, path::Path};

use crate::{pushshift::PSError, scraperclient::nodestructs::Node};

/// Reads all Nodes from a CSV file into a HashSet.
/// Fails if the file cannot be read but reports errors while reading instead of failing.
pub fn read_nodes<P>(path: P) -> Result<HashSet<Node>, PSError>
where
    P: AsRef<Path>,
{
    // Partition errors so that invalid rows don't cause the entire operation to fail.
    let (nodes, errors): (Vec<_>, Vec<_>) = Reader::from_path(path)?
        .deserialize()
        .partition(|result| result.is_ok());

    for error in errors {
        let error = error.unwrap_err();

        error!(
            "Failed to parse row: {row:?}. Reason: {error}",
            row = error
                .position()
                .map(|position| position.record())
                .unwrap_or(u64::MAX),
            error = error
        );
    }

    Ok(nodes.into_iter().map(|result| result.unwrap()).collect())
}

/// Writes all nodes to `path`.
pub fn write_nodes<P>(path: P, nodes: &HashSet<Node>) -> Result<(), PSError>
where
    P: AsRef<Path>,
{
    let mut writer = Writer::from_path(path)?;
    for node in nodes.iter() {
        if let Some(error) = writer.serialize(node).err() {
            error!("Failed to write a row: {error}");
        }
    }

    Ok(writer.flush()?)
}
