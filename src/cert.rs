use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, SanType};
use std::net::IpAddr;

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

    // Subject Alternative Names (SAN)
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
