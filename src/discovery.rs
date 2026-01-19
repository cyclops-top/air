use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use mdns_sd::{ServiceDaemon, ServiceInfo, ServiceEvent};

pub const SERVICE_TYPE: &str = "_air-share._tcp.local.";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveryMsg {
    pub id: String,         // Maps to fullname
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
    pub scheme: String,
    pub is_online: bool,
}

/// Registers the service and returns the daemon and the fullname.
pub fn register_service(msg: &DiscoveryMsg) -> anyhow::Result<(ServiceDaemon, String)> {
    let mdns = ServiceDaemon::new()?;
    
    let service_type = SERVICE_TYPE;
    let instance_name = &msg.name;
    let host_name = format!("{}.local.", msg.name.replace(' ', "-"));
    let port = msg.port;
    
    let properties = [
        ("id", msg.id.as_str()),
        ("scheme", msg.scheme.as_str()),
        ("ver", "1.0"),
    ];

    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        &host_name,
        msg.ip,
        port,
        &properties[..],
    )?;

    let fullname = service_info.get_fullname().to_string();
    mdns.register(service_info)?;
    
    Ok((mdns, fullname))
}

/// Continuously listens for mDNS services and sends updates through a channel.
pub async fn listen_discovery(
    tx: tokio::sync::mpsc::Sender<DiscoveryMsg>,
    mut shutdown_rx: tokio::sync::oneshot::Receiver<()>, 
) -> anyhow::Result<()> {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse(SERVICE_TYPE)?;
    
    // Bridge to convert blocking recv to async
    let (bridge_tx, mut bridge_rx) = tokio::sync::mpsc::channel::<ServiceEvent>(100);

    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            if bridge_tx.blocking_send(event).is_err() {
                break;
            }
        }
    });

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => break,
            event_opt = bridge_rx.recv() => {
                let event = match event_opt {
                    Some(e) => e,
                    None => break,
                };
                
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let fullname = info.get_fullname().to_string();
                        let scheme = info.get_property_val_str("scheme").unwrap_or("http").to_string();
                        let name = fullname.split('.')
                            .next()
                            .unwrap_or(&fullname)
                            .replace('\\', "")
                            .to_string();
                        
                        let ip_scoped = info.get_addresses().iter()
                            .find(|ip| ip.is_ipv4())
                            .or_else(|| info.get_addresses().iter().next())
                            .cloned();
                        
                        let ip: IpAddr = match ip_scoped {
                            Some(scoped) => scoped.to_string().parse().unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
                            None => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                        };

                        let msg = DiscoveryMsg {
                            id: fullname,
                            name,
                            ip,
                            port: info.get_port(),
                            scheme,
                            is_online: true,
                        };
                        let _ = tx.send(msg).await;
                    }
                    ServiceEvent::ServiceRemoved(_service_type, fullname) => {
                        let msg = DiscoveryMsg {
                            id: fullname.clone(),
                            name: "".to_string(),
                            ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                            port: 0,
                            scheme: "".to_string(),
                            is_online: false,
                        };
                        let _ = tx.send(msg).await;
                    }
                    _ => {}
                }
            }
        }
    }
    
    let _ = mdns.stop_browse(SERVICE_TYPE);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_msg_serialization() {
        let msg = DiscoveryMsg {
            id: "test.air.local.".to_string(),
            name: "test-node".to_string(),
            ip: "192.168.1.100".parse().unwrap(),
            port: 8080,
            scheme: "http".to_string(),
            is_online: true,
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: DiscoveryMsg = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg.id, deserialized.id);
        assert_eq!(msg.is_online, deserialized.is_online);
    }
}
