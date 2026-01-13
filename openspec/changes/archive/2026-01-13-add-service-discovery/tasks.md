# Tasks: Implement Service Discovery

- [x] Add dependencies to `Cargo.toml` (`socket2`, `hostname`, `serde`, `serde_json`). <!-- id: 0 -->
- [x] Create `src/discovery.rs` and define `DiscoveryMsg` struct. <!-- id: 1 -->
- [x] Implement `start_broadcast` function in `src/discovery.rs`. <!-- id: 2 -->
- [x] Implement `run_discovery` function in `src/discovery.rs` with socket options. <!-- id: 3 -->
- [x] Integrate `air discover` command in `src/main.rs`. <!-- id: 4 -->
- [x] Hook `start_broadcast` into the server startup logic in `src/main.rs`. <!-- id: 5 -->
- [x] Verify discovery functionality between two terminal windows. <!-- id: 6 -->
- [x] Add unit tests for `DiscoveryMsg` serialization/deserialization. <!-- id: 7 -->
