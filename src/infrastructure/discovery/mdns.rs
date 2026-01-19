use async_trait::async_trait;
use mdns_sd::{ServiceDaemon, ServiceInfo, ServiceEvent};
use std::net::{IpAddr, Ipv4Addr};
use crate::domain::models::DiscoveryMsg;
use crate::domain::traits::DiscoveryProvider;

pub struct MdnsDiscoveryProvider {
    daemon: ServiceDaemon,
}

impl MdnsDiscoveryProvider {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            daemon: ServiceDaemon::new()?,
        })
    }
}

#[async_trait]
impl DiscoveryProvider for MdnsDiscoveryProvider {
    fn register(&self, msg: &DiscoveryMsg) -> anyhow::Result<String> {
        let service_type = "_air-share._tcp.local.";
        let instance_name = &msg.name;
        let host_name = format!("{}.local.", msg.name.replace(' ', "-"));
        
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
            msg.port,
            &properties[..],
        )?;

        let fullname = service_info.get_fullname().to_string();
        self.daemon.register(service_info)?;
        Ok(fullname)
    }

    fn unregister(&self, fullname: &str) -> anyhow::Result<()> {
        self.daemon.unregister(fullname)?;
        Ok(())
    }

    async fn listen(&self, tx: tokio::sync::mpsc::Sender<DiscoveryMsg>, mut shutdown_rx: tokio::sync::oneshot::Receiver<()>) -> anyhow::Result<()> {
        let service_type = "_air-share._tcp.local.";
        let receiver = self.daemon.browse(service_type)?;
        
        let (bridge_tx, mut bridge_rx) = tokio::sync::mpsc::channel::<ServiceEvent>(100);

        std::thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                if bridge_tx.blocking_send(event).is_err() { break; }
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
                            let name = fullname.split('.').next().unwrap_or(&fullname).replace('\\', "").to_string();
                            
                            let ip_scoped = info.get_addresses().iter()
                                .find(|ip| ip.is_ipv4())
                                .or_else(|| info.get_addresses().iter().next())
                                .cloned();
                            
                            let ip: IpAddr = match ip_scoped {
                                Some(scoped) => scoped.to_string().parse().unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
                                None => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                            };

                            let _ = tx.send(DiscoveryMsg {
                                id: fullname,
                                name,
                                ip,
                                port: info.get_port(),
                                scheme,
                                is_online: true,
                            }).await;
                        }
                        ServiceEvent::ServiceRemoved(_st, fullname) => {
                            let _ = tx.send(DiscoveryMsg {
                                id: fullname,
                                name: "".to_string(),
                                ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                                port: 0,
                                scheme: "".to_string(),
                                is_online: false,
                            }).await;
                        }
                        _ => {}
                    }
                }
            }
        }
        self.daemon.stop_browse(service_type)?;
        Ok(())
    }
}
