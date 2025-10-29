// Peer Tracker Module
// Responsible for tracking peers for each torrent (announce, list).
// Uses a HashMap keyed by info_hash, with Vec<Peer> for each torrent.
use crate::models::{Peer, PeerStatus};
use std::collections::HashMap;

pub struct Tracker {
    peers: HashMap<String, Vec<Peer>>,
}

impl Tracker {
    /// Create a new, empty Tracker.
    pub fn new() -> Self {
        Tracker {
            peers: HashMap::new(),
        }
    }

    /// Announce a peer for a torrent.
    /// Returns true if the peer was added (not already present).
    pub fn announce_peer(&mut self, peer: Peer) -> bool {
        let entry = self
            .peers
            .entry(peer.info_hash.clone())
            .or_insert_with(Vec::new);
        if entry.iter().any(|p| p.peer_id == peer.peer_id) {
            false // Peer already announced for this torrent
        } else {
            entry.push(peer);
            true
        }
    }

    /// List all peers for a given torrent info_hash.
    pub fn list_peers(&self, info_hash: &str) -> Vec<&Peer> {
        self.peers
            .get(info_hash)
            .map(|v| v.iter().collect())
            .unwrap_or_else(Vec::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Peer, PeerStatus};

    // Helper to create a sample peer for tests.
    fn sample_peer(peer_id: &str, info_hash: &str, status: PeerStatus) -> Peer {
        Peer {
            peer_id: peer_id.to_string(),
            info_hash: info_hash.to_string(),
            status,
        }
    }

    #[test]
    fn test_announce_and_list_peers() {
        let mut tracker = Tracker::new();
        let peer1 = sample_peer("peer1", "abc123", PeerStatus::Seeding);
        let peer2 = sample_peer("peer2", "abc123", PeerStatus::Leeching);
        assert!(tracker.announce_peer(peer1.clone()));
        assert!(tracker.announce_peer(peer2.clone()));
        assert!(!tracker.announce_peer(peer1.clone())); // duplicate
        let peers = tracker.list_peers("abc123");
        assert_eq!(peers.len(), 2);
        assert!(peers.iter().any(|p| p.peer_id == "peer1"));
        assert!(peers.iter().any(|p| p.peer_id == "peer2"));
    }

    #[test]
    fn test_list_peers_empty() {
        let tracker = Tracker::new();
        let peers = tracker.list_peers("notfound");
        assert!(peers.is_empty());
    }
}
// ...existing code...
