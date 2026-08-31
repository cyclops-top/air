use crate::domain::models::DiscoveryMsg;
use crate::domain::traits::DiscoveryProvider;
use mdns_sd::{ServiceDaemon, ServiceInfo, ServiceEvent};

pub struct MdnsDiscoveryProvider {
    daemon: ServiceDaemon,
}

impl MdnsDiscoveryProvider {
    pub fn new() -> anyhow::Result<Self> {
        let daemon = ServiceDaemon::new()?;
        Ok(Self { daemon })
    }
}

impl DiscoveryProvider for MdnsDiscoveryProvider {
    fn register_service(&self, msg: &DiscoveryMsg) -> anyhow::Result<String> {
        let service_type = "_air-share._tcp.local.";
        let instance_name = &msg.name;
        let host_name = format!("{}.local.", msg.name.replace(' ', "-"));

        // 始终使用局域网 IP，如果传入的不是有效局域网 IP，则获取本地局域网 IP
        let ip = get_local_lan_ip().unwrap_or_else(|| msg.ip.to_string());

        let properties = [
            ("id", msg.id.as_str()),
            ("scheme", msg.scheme.as_str()),
        ];

        let service_info = ServiceInfo::new(
            service_type,
            instance_name,
            &host_name,
            ip,
            msg.port,
            &properties[..],
        )?.enable_addr_auto();

        self.daemon.register(service_info)?;
        let fullname = format!("{}.{}", instance_name, service_type);
        Ok(fullname)
    }

    fn unregister_service(&self, fullname: &str) -> anyhow::Result<()> {
        self.daemon.unregister(fullname)?;
        Ok(())
    }

    fn start_discovery(&self, tx: tokio::sync::mpsc::Sender<DiscoveryMsg>) -> anyhow::Result<()> {
        let service_type = "_air-share._tcp.local.";
        let receiver = self.daemon.browse(service_type)?;

        // Spawn a blocking loop to handle mDNS events, as browse() returns a receiver
        let tx_clone = tx.clone();
        
        // 由于 start_discovery 在 trait 中定义为同步返回 Result，但需要长期运行，
        // 我们在这里启动一个后台任务。注意：Trait 定义中没有 shutdown_rx，
        // 所以我们暂时让它一直运行，直到通道关闭（tx被drop）。
        std::thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let scheme = info.get_property("scheme").map(|p| p.val_str()).unwrap_or("http").to_string();
                        let id = info.get_property("id").map(|p| p.val_str()).unwrap_or("unknown").to_string();

                        // 优先选择 IPv4 私有地址，其次是 IPv6 ULA
                        let mut selected_ip: Option<std::net::IpAddr> = None;

                        for addr in info.get_addresses() {
                            let ip_str = addr.to_string();
                            let ip_clean = if let Some(idx) = ip_str.find('%') {
                                &ip_str[..idx]
                            } else {
                                &ip_str
                            };

                            if let Ok(ip_addr) = ip_clean.parse::<std::net::IpAddr>() {
                                // 跳过链路本地地址 (fe80::/10)
                                if is_link_local_ip(&ip_addr) {
                                    continue;
                                }

                                // 优先选择 IPv4 私有地址
                                if let std::net::IpAddr::V4(_) = ip_addr {
                                    if is_private_ip(&ip_addr) {
                                        selected_ip = Some(ip_addr);
                                        break;
                                    }
                                }
                            }
                        }

                        // 如果没找到合适的 IP，再尝试找任何非链路本地的 IPv4
                        if selected_ip.is_none() {
                            for addr in info.get_addresses() {
                                let ip_str = addr.to_string();
                                let ip_clean = if let Some(idx) = ip_str.find('%') {
                                    &ip_str[..idx]
                                } else {
                                    &ip_str
                                };

                                if let Ok(ip_addr) = ip_clean.parse::<std::net::IpAddr>() {
                                    if !is_link_local_ip(&ip_addr) {
                                        selected_ip = Some(ip_addr);
                                        break;
                                    }
                                }
                            }
                        }

                        if let Some(ip) = selected_ip {
                            let msg = DiscoveryMsg {
                                id,
                                name: info.get_fullname().to_string(),
                                ip,
                                port: info.get_port(),
                                scheme,
                                is_online: true,
                            };
                            let _ = tx_clone.blocking_send(msg);
                        }
                    }
                    ServiceEvent::ServiceRemoved(_, fullname) => {
                        let msg = DiscoveryMsg {
                            id: fullname.clone(), // Use fullname as ID for removal
                            name: fullname,
                            ip: "127.0.0.1".parse().unwrap(),
                            port: 0,
                            scheme: "http".to_string(),
                            is_online: false,
                        };
                        let _ = tx_clone.blocking_send(msg);
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
}

/// 获取本地局域网 IP 地址
/// 优先返回私有网络地址，回环地址和链路本地地址
fn get_local_lan_ip() -> Option<String> {
    let network_interfaces = if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
        interfaces
    } else {
        return None;
    };

    for (_name, ip) in network_interfaces {
        // 跳过回环地址
        if ip.is_loopback() {
            continue;
        }

        // 检查是否是私有网络地址
        if is_private_ip(&ip) {
            return Some(ip.to_string());
        }
    }

    None
}

/// 检查是否是私有网络 IP
fn is_private_ip(ip: &std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // 10.0.0.0/8
            if octets[0] == 10 {
                return true;
            }
            // 172.16.0.0/12
            if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 {
                return true;
            }
            // 192.168.0.0/16
            if octets[0] == 192 && octets[1] == 168 {
                return true;
            }
            false
        }
        std::net::IpAddr::V6(ipv6) => {
            // fe80::/10 是链路本地地址，不使用
            // fc00::/7 是唯一本地地址 (ULA)，可以使用
            let segments = ipv6.segments();
            (segments[0] & 0xfe00) == 0xfc00
        }
    }
}

/// 检查是否是链路本地 IP (IPv4 169.254.x.x 或 IPv6 fe80::/10)
fn is_link_local_ip(ip: &std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // 169.254.0.0/16
            octets[0] == 169 && octets[1] == 254
        }
        std::net::IpAddr::V6(ipv6) => {
            // fe80::/10
            let segments = ipv6.segments();
            (segments[0] & 0xffc0) == 0xfe80
        }
    }
}