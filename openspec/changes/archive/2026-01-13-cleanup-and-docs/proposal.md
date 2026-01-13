# Proposal: Cleanup and Documentation Update

## Goal
Fix compiler warnings, remove unused code, and update the project documentation (README.md) to reflect recent feature additions like Service Discovery.

## Motivation
Maintaining a warning-free codebase is essential for code quality and catching actual bugs. The documentation currently lacks details on the new `discover` command, which is a key feature for user experience.

## Scope
- Remove the unused `run_discovery` function in `src/discovery.rs`.
- Update `README.md` with:
  - New "Service Discovery" feature description.
  - Instructions for the `discover` command.
  - TUI controls for discovery.
- General code formatting and minor organization if necessary.

## Out of Scope
- Large-scale architectural refactoring.
- Adding new features.
