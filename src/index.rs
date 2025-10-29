// Torrent Index Module
// Responsible for managing torrents in-memory: add, list, search, and get by info_hash.
// Uses a HashMap for efficient lookup.
use crate::models::Torrent;
use std::collections::HashMap;

// The Index struct holds all torrents, keyed by their info_hash.
pub struct Index {
    torrents: HashMap<String, Torrent>,
}

impl Index {
    /// Create a new, empty Index.
    pub fn new() -> Self {
        Index {
            torrents: HashMap::new(),
        }
    }

    /// Add a torrent to the index.
    /// Returns true if added, false if info_hash already exists.
    pub fn add_torrent(&mut self, torrent: Torrent) -> bool {
        let info_hash = torrent.info_hash.clone();
        if self.torrents.contains_key(&info_hash) {
            false
        } else {
            self.torrents.insert(info_hash, torrent);
            true
        }
    }

    /// List all torrents in the index.
    pub fn list_torrents(&self) -> Vec<&Torrent> {
        self.torrents.values().collect()
    }

    /// Search torrents by name or info_hash substring.
    pub fn search_torrents(&self, query: &str) -> Vec<&Torrent> {
        self.torrents
            .values()
            .filter(|t| t.name.contains(query) || t.info_hash.contains(query))
            .collect()
    }

    /// Get a torrent by its info_hash.
    pub fn get_torrent(&self, info_hash: &str) -> Option<&Torrent> {
        self.torrents.get(info_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Torrent;

    fn sample_torrent(name: &str, info_hash: &str) -> Torrent {
        Torrent {
            name: name.to_string(),
            info_hash: info_hash.to_string(),
            // Helper to create a sample torrent for tests.
            file_size: 1234,
            added_at: "2025-10-29T12:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_add_and_get_torrent() {
        let mut index = Index::new();
        let torrent = sample_torrent("Test", "abc123");
        // Test adding a torrent and retrieving it by info_hash.
        assert!(index.add_torrent(torrent.clone()));
        assert!(!index.add_torrent(torrent.clone())); // duplicate
        let found = index.get_torrent("abc123");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");
    }

    #[test]
    fn test_list_torrents() {
        let mut index = Index::new();
        index.add_torrent(sample_torrent("A", "hashA"));
        // Test listing all torrents.
        index.add_torrent(sample_torrent("B", "hashB"));
        let list = index.list_torrents();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_search_torrents() {
        let mut index = Index::new();
        index.add_torrent(sample_torrent("Ubuntu", "hash1"));
        // Test searching for torrents by name or info_hash.
        index.add_torrent(sample_torrent("Fedora", "hash2"));
        let results = index.search_torrents("Ubuntu");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Ubuntu");
        let results = index.search_torrents("hash2");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Fedora");
    }
}
// ...existing code...
