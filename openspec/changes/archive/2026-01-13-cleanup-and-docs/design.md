# Design: Cleanup and Documentation Update

## Implementation Strategy

### 1. Code Cleanup
- **Warning Fix**: Remove `run_discovery` from `src/discovery.rs` as it has been replaced by the interactive `listen_discovery` logic.
- **Dead Code**: Scan for any other unused imports or functions that `cargo check` might have missed in less common configurations (though currently only one warning is reported).

### 2. Documentation (README.md)
The README needs to be updated to include the "Service Discovery" feature.

#### Added Sections:
- **Service Discovery**: Explain how Air nodes find each other.
- **`discover` command**: 
  - Usage: `air discover`.
  - Behavior: Continuous search with an interactive TUI.
  - Controls: [Enter] to open URL, [q] to quit.

### 3. Verification
- Run `cargo check` to ensure zero warnings.
- Manual inspection of the updated `README.md`.
