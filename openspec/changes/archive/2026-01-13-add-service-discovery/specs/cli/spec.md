# CLI Specification Delta

## ADDED Requirements

### Requirement: Service Discovery Command
The Air CLI MUST support a sub-command to discover other Air nodes on the local network.

#### Scenario: Running Discovery
- **Given** the user runs `air discover`.
- **When** there are active Air servers broadcasting on the local network.
- **Then** the CLI MUST bind to a discovery port (e.g., 29888) and listen for UDP packets.
- **And** it MUST print a formatted list of discovered services, including:
  - Hostname
  - IP Address
  - Full URL (scheme://ip:port)
- **And** it MUST continue listening until the user interrupts (Ctrl+C).

### Requirement: Multi-Instance Support
The discovery command MUST allow multiple instances to run simultaneously on the same machine.

#### Scenario: Concurrent Discoveries
- **Given** one instance of `air discover` is already running.
- **When** the user starts another instance of `air discover`.
- **Then** the second instance MUST NOT fail with "Address already in use".
- **And** both instances MUST receive and display the same broadcast information.
