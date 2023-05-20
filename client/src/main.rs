use quic_rpc::RpcClient;
use quinn::{ClientConfig, Endpoint};
use std::{io, net::SocketAddr, sync::Arc};
use types::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("client started");
    let server_addr: SocketAddr = "127.0.0.1:20542".parse()?;
    let endpoint = make_insecure_client_endpoint("0.0.0.0:0".parse()?)?;
    let client = quic_rpc::transport::quinn::QuinnConnection::new(
        endpoint,
        server_addr,
        "localhost".to_string(),
    );
    let client = RpcClient::<ThermostatService, _>::new(client);
    println!("calling rpc");
    let res = client.rpc(SensorReading(69.7)).await;
    println!("rpc result: sensor reading 69.7 = {:?}", res.unwrap());
    Ok(())
}

pub fn make_insecure_client_endpoint(bind_addr: SocketAddr) -> io::Result<Endpoint> {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();

    let client_cfg = ClientConfig::new(Arc::new(crypto));
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

struct SkipServerVerification;
impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
