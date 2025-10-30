// Import the index, models, and tracker modules so we can use their structs and functions.
mod index;
mod models;
mod tracker;

// Bring the main types and structs into scope for easier access.
use index::Index;
use models::{Peer, PeerStatus, Torrent};
use tracker::Tracker;

fn main() {
    // Create a new torrent index (for storing torrents) and a tracker (for managing peers).
    let mut index = Index::new();
    let mut tracker = Tracker::new();

    // Create and add a sample torrent to the index.
    let torrent = Torrent {
        name: "Example Torrent".to_string(),          // Torrent name
        info_hash: "abc123".to_string(),              // Unique identifier for the torrent
        file_size: 1024 * 1024 * 700,                 // File size in bytes (700 MB)
        added_at: "2025-10-29T12:00:00Z".to_string(), // ISO timestamp
    };
    index.add_torrent(torrent); // Add the torrent to the index

    // Create two peers and announce them to the tracker for the given torrent.
    let peer1 = Peer {
        peer_id: "peer1".to_string(),    // Unique peer ID
        info_hash: "abc123".to_string(), // Torrent info_hash this peer is associated with
        status: PeerStatus::Seeding,     // Peer is seeding
    };
    let peer2 = Peer {
        peer_id: "peer2".to_string(),    // Unique peer ID
        info_hash: "abc123".to_string(), // Torrent info_hash this peer is associated with
        status: PeerStatus::Leeching,    // Peer is leeching
    };
    tracker.announce_peer(peer1); // Announce peer1
    tracker.announce_peer(peer2); // Announce peer2

    // List all torrents in the index and print them.
    print!("Torrents:\n");
    for t in index.list_torrents() {
        println!("  Name: {}", t.name); // Print each torrent struct
        println!("  Info Hash: {}", t.info_hash);
        println!("  Size: {} bytes", t.file_size);
        println!("  Added At: {}", t.added_at);
        println!();
    }

    // List all peers for the torrent with info_hash "abc123" and print them.
    println!("\nPeers for abc123:");
    for p in tracker.list_peers("abc123") {
        println!("  Peer ID: {}", p.peer_id); // Print each peer struct
        println!("  Status: {:?}", p.status);
        println!();
    }
}
