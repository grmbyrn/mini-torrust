# mini-torrust

A simplified BitTorrent index and tracker written in Rust. This project is designed for learning and experimentation, inspired by the architecture and philosophy of Torrust.

## Overview

`mini-torrust` helps you keep track of files (torrents) and the people (peers) sharing them. It provides:

- An in-memory index of torrents
- A tracker for peers associated with each torrent
- JSON-based persistence for saving and loading data
- Unit tests for reliability

## Features

- **Torrent Index:**
  - Add new torrents
  - List all torrents
  - Search torrents by name or info hash
- **Peer Tracker:**
  - Announce peers for a torrent
  - List all peers for a torrent
- **Persistence:**
  - Save and load torrents and peers to/from JSON files
- **Testing:**
  - Unit tests for all modules

## Project Structure

- `src/models.rs` — Core data structures (`Torrent`, `Peer`, `PeerStatus`)
- `src/index.rs` — Torrent index logic
- `src/tracker.rs` — Peer tracking logic
- `src/db.rs` — Persistence (save/load to JSON)
- `src/main.rs` — Example usage and wiring

## How to Run

1. **Build and run the app:**

   ```sh
   cargo run
   ```

   This will print the sample torrent and peers to the terminal.

2. **Run the tests:**
   ```sh
   cargo test
   ```
   This will run all unit tests, including persistence tests in `db.rs`.

## Example Output

```
Torrents:
  Name: Example Torrent
  Info Hash: abc123
  Size: 734003200 bytes
  Added At: 2025-10-29T12:00:00Z

Peers for abc123:
  Peer ID: peer1
  Status: Seeding

  Peer ID: peer2
  Status: Leeching
```

## Dependencies

- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) for serialization

## Next Steps

Here are some ideas for expanding the project:

- **Command-Line Interface (CLI):**
  - Add interactive commands for adding/searching torrents and announcing/listing peers
- **Web API:**
  - Expose index and tracker via HTTP using `axum` or `actix-web`
- **Database Support:**
  - Add SQLite persistence with `sqlx` or `diesel`
- **More Tests:**
  - Expand integration and API endpoint tests
- **Documentation:**
  - Add more detailed usage and contribution guidelines

Feel free to fork, experiment, and contribute!
