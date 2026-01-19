use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket as StdUdpSocket};
use std::time::Duration;
use tokio::time::sleep;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

pub const DISCOVERY_PORTS: [u16; 3] = [29888, 29889, 29890];
const BROADCAST_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveryMsg {
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
    pub scheme: String,
}

/// Starts a background task that broadcasts the service presence.
pub async fn start_broadcast(msg: DiscoveryMsg) {
    tokio::spawn(async move {
        // We use a regular std::net::UdpSocket for broadcasting as it's simpler for this use case
        let socket = StdUdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket for broadcasting");
        socket.set_broadcast(true).expect("Failed to set broadcast on UDP socket");

        loop {
            let payload = serde_json::to_vec(&msg).expect("Failed to serialize discovery message");
            for port in DISCOVERY_PORTS {
                let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), port);
                let _ = socket.send_to(&payload, addr);
            }
            sleep(BROADCAST_INTERVAL).await;
        }
    });
}

/// Continuously listens for discovery messages and sends updates through a channel.
pub async fn listen_discovery(tx: tokio::sync::mpsc::Sender<DiscoveryMsg>) -> anyhow::Result<()> {
    let mut sockets = Vec::new();
    for &port in &DISCOVERY_PORTS {
        if let Ok(socket) = create_discovery_socket(port) {
            socket.set_nonblocking(true)?;
            let tokio_socket = tokio::net::UdpSocket::from_std(socket.into())?;
            sockets.push(tokio_socket);
        }
    }

    if sockets.is_empty() {
        return Err(anyhow::anyhow!("Failed to bind to any discovery ports"));
    }

    let mut buf = [0u8; 1024];
    loop {
        for socket in &sockets {
            while let Ok((len, _addr)) = socket.try_recv_from(&mut buf) {
                if let Ok(msg) = serde_json::from_slice::<DiscoveryMsg>(&buf[..len]) {
                    let _ = tx.send(msg).await;
                }
            }
        }
        sleep(Duration::from_millis(100)).await;
    }
}

fn create_discovery_socket(port: u16) -> anyhow::Result<Socket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    
    // SO_REUSEADDR and SO_REUSEPORT are essential for multiple instances to listen on the same discovery port.
    // On Windows, SO_REUSEADDR has different semantics, and SO_REUSEPORT is not available.
    #[cfg(not(windows))]
    socket.set_reuse_address(true)?;
    
    #[cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos")))]
    let _ = socket.set_reuse_port(true); // Ignore error if not supported
    
    let addr: SockAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port).into();
    socket.bind(&addr)?;
    
    Ok(socket)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_msg_serialization() {
        let msg = DiscoveryMsg {
            name: "test-node".to_string(),
            ip: "192.168.1.100".parse().unwrap(),
            port: 8080,
            scheme: "http".to_string(),
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: DiscoveryMsg = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg.name, deserialized.name);
        assert_eq!(msg.ip, deserialized.ip);
        assert_eq!(msg.port, deserialized.port);
        assert_eq!(msg.scheme, deserialized.scheme);
    }
}
