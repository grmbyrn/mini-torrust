// Created models.rs: This file holds the core data structures (Torrent, Peer, PeerStatus) that represent torrents and peers in your application. Keeping them in a separate module makes your code organized and reusable.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Torrent {
    pub name: String,
    pub info_hash: String,
    pub file_size: u64,
    pub added_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerStatus {
    Seeding,
    Leeching,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Peer {
    pub peer_id: String,
    pub info_hash: String,
    pub status: PeerStatus,
}

// Added unit tests: Testing your models ensures they work as expected and helps catch errors early. It’s a best practice in Rust and most modern languages.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torrent_creation() {
        let t = Torrent {
            name: "Test".to_string(),
            info_hash: "abc123".to_string(),
            file_size: 1000,
            added_at: "2025-10-29T12:00:00Z".to_string(),
        };
        assert_eq!(t.name, "Test");
    }

    #[test]
    fn test_peer_status_enum() {
        let s = PeerStatus::Seeding;
        let l = PeerStatus::Leeching;
        assert_ne!(s, l);
    }

    #[test]
    fn test_peer_creation() {
        let p = Peer {
            peer_id: "peer1".to_string(),
            info_hash: "abc123".to_string(),
            status: PeerStatus::Seeding,
        };
        assert_eq!(p.status, PeerStatus::Seeding);
    }
}
