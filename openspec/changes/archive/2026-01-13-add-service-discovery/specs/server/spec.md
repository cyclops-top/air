# Server Specification Delta

## ADDED Requirements

### Requirement: Background Presence Broadcasting
The Air server MUST periodically broadcast its presence to the local network.

#### Scenario: Periodic UDP Broadcast
- **Given** the Air server is started.
- **When** the server is running.
- **Then** it MUST spawn a background task that sends UDP broadcast packets every 3 seconds.
- **And** the packets MUST be sent to the broadcast address `255.255.255.255` on ports 29888, 29889, and 29890.
- **And** the payload MUST be a JSON object containing `name` (hostname), `ip` (LAN IP), `port` (HTTP port), and `scheme` (http/https).

### Requirement: Resilient Broadcasting
The Air server MUST remain stable even if broadcasting fails.

#### Scenario: Send Failure Handling
- **Given** the broadcasting task is running.
- **When** a single send operation fails (e.g., network interface down).
- **Then** the task MUST NOT crash and SHOULD continue trying in the next interval.
