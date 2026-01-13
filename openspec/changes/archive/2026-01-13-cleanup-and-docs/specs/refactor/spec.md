# Specification: Refactor Cleanup

## Capabilities: refactor

## REMOVED Requirements

### Requirement: Remove Unused Discovery Logic
Fix the compiler warning regarding dead code in the discovery module.

#### Scenario: Remove run_discovery
- **Given** the current implementation of `src/discovery.rs` contains an unused `run_discovery` function.
- **When** the `run_discovery` function is removed.
- **Then** `cargo check` should no longer report a "dead_code" warning for this function.