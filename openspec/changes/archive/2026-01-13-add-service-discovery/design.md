# Design: Service Discovery

## Architecture
The service discovery system consists of two main components: the **Broadcaster** (Server-side) and the **Listener** (Client-side).

### 1. Protocol Details
- **Transport**: UDP Broadcast on IPv4.
- **Ports**: `[29888, 29889, 29890]`. We attempt to use these ports to avoid conflicts and increase reliability.
- **Frequency**: Broadcast every 3 seconds.
- **Payload**: JSON format.
  ```json
  {
    "name": "Hostname",
    "ip": "192.168.1.x",
    "port": 9000,
    "scheme": "http"
  }
  ```

### 2. Implementation Strategy

#### Dependencies
- `socket2`: For advanced socket configuration (`SO_REUSEPORT`).
- `serde`/`serde_json`: For payload serialization.
- `hostname`: To get the device name.
- `local-ip-address`: To get the primary LAN IP.

#### Server Component (Broadcaster)
- Spawned as a background `tokio` task.
- Binds to `0.0.0.0:0`.
- Sends to `255.255.255.255:<port>` for each port in the target list.

#### Client Component (Listener)
- Triggered by `air discover`.
- Binds to one of the discovery ports using `SO_REUSEADDR` and `SO_REUSEPORT` to allow multiple listeners on the same machine (important for macOS compatibility).
- Formats and displays received messages in a table or list.

### 3. Error Handling
- Failures to bind to a discovery port should result in a clear error message.
- Failures to send a single broadcast packet are ignored (best-effort).
- Malformed JSON packets from other sources are ignored.

### 4. Integration
- `src/discovery.rs`: New module for all discovery logic.
- `src/main.rs`: CLI integration for the `discover` command and server hook.
