// This is the database for saving and loading torrents and peers.
// We use serde to wrangle our structs into JSON and back, so nothing gets lost.

use crate::index::Index;
use crate::models::{Peer, Torrent};
use crate::tracker::Tracker;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Error as IoError};

// First, we make sure our structs can be serialized.
// Add serde derives to Torrent and Peer in models.rs if they aren't there yet.

// Save the whole torrent herd to a JSON file.
pub fn save_index(index: &Index, path: &str) -> Result<(), IoError> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let writer = BufWriter::new(file);
    // We serialize just the values.
    let torrents: Vec<&Torrent> = index.list_torrents();
    serde_json::to_writer(writer, &torrents)?;
    Ok(())
}

// Load the index back from the JSON file.
pub fn load_index(path: &str) -> Result<Index, IoError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let torrents: Vec<Torrent> = serde_json::from_reader(reader)?;
    let mut index = Index::new();
    for t in torrents {
        index.add_torrent(t);
    }
    Ok(index)
}

// Save all the tracker’s peers to a JSON file.
pub fn save_tracker(tracker: &Tracker, path: &str) -> Result<(), IoError> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let writer = BufWriter::new(file);
    // Flatten the tracker’s map into a list of peers.
    let mut all_peers = Vec::new();
    for peers in tracker.peers.values() {
        for peer in peers {
            all_peers.push(peer);
        }
    }
    serde_json::to_writer(writer, &all_peers)?;
    Ok(())
}

// Load all the tracker’s peers from a JSON file.
pub fn load_tracker(path: &str) -> Result<Tracker, IoError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let peers: Vec<Peer> = serde_json::from_reader(reader)?;
    let mut tracker = Tracker::new();
    for p in peers {
        tracker.announce_peer(p);
    }
    Ok(tracker)
}

// Tests for our data
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Peer, PeerStatus, Torrent};
    use std::fs;

    #[test]
    fn test_save_and_load_index() {
        let mut index = Index::new();
        let torrent = Torrent {
            name: "Cowboy Movie".to_string(),
            info_hash: "hashcowboy".to_string(),
            file_size: 123,
            added_at: "2025-10-30T12:00:00Z".to_string(),
        };
        index.add_torrent(torrent);
        let path = "test_index.json";
        save_index(&index, path).unwrap();
        let loaded = load_index(path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(loaded.list_torrents().len(), 1);
    }

    #[test]
    fn test_save_and_load_tracker() {
        let mut tracker = Tracker::new();
        let peer = Peer {
            peer_id: "cowpoke".to_string(),
            info_hash: "hashcowboy".to_string(),
            status: PeerStatus::Seeding,
        };
        tracker.announce_peer(peer);
        let path = "test_tracker.json";
        save_tracker(&tracker, path).unwrap();
        let loaded = load_tracker(path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(loaded.list_peers("hashcowboy").len(), 1);
    }
}
