use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, SanType};
use std::net::IpAddr;
use axum_server::tls_rustls::RustlsConfig;

pub struct GeneratedCert {
    pub cert_pem: String,
    pub key_pem: String,
}

pub fn generate_self_signed(lan_ip: IpAddr) -> anyhow::Result<GeneratedCert> {
    let mut params = CertificateParams::default();
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, "Air Local File Server");
    dn.push(DnType::OrganizationName, "Air Project");
    params.distinguished_name = dn;

    params.subject_alt_names = vec![
        SanType::IpAddress(lan_ip),
        SanType::IpAddress("127.0.0.1".parse().unwrap()),
        SanType::DnsName("localhost".to_string().try_into().unwrap()),
    ];

    let key_pair = KeyPair::generate()?;
    let cert = params.self_signed(&key_pair)?;

    Ok(GeneratedCert {
        cert_pem: cert.pem(),
        key_pem: key_pair.serialize_pem(),
    })
}

pub fn get_config(lan_ip: IpAddr) -> anyhow::Result<RustlsConfig> {
    let cert = generate_self_signed(lan_ip)?;
    // RustlsConfig::from_pem expects bytes
    let config = tokio::runtime::Runtime::new()?.block_on(async {
        RustlsConfig::from_pem(
            cert.cert_pem.into_bytes(),
            cert.key_pem.into_bytes()
        ).await
    })?;
    Ok(config)
}