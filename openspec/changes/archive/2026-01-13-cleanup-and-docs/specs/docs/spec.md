# Specification: Documentation Update

## Capabilities: docs

## MODIFIED Requirements

### Requirement: Update README with Service Discovery
The `README.md` **MUST** reflect the newly added service discovery capabilities.

#### Scenario: Add Service Discovery Section
- **Given** the current `README.md` does not mention service discovery.
- **When** information about UDP-based discovery and the `discover` command is added.
- **Then** users should be able to understand how to use the discovery feature from the documentation.

### Requirement: Document Discover TUI Controls
TUI controls specifically for the discovery mode **SHALL** be documented.

#### Scenario: List Discovery Controls
- **Given** the `README.md` only lists server-mode TUI controls.
- **When** controls for `discover` mode ([Enter] to open, [Up/Down] to navigate) are added.
- **Then** the documentation is complete for all interactive modes.
