use quic_rpc::server::run_server_loop;
use quinn::{Endpoint, ServerConfig};
use std::net::SocketAddr;
use std::sync::Arc;

use types::*;

#[derive(Clone)]
pub struct Thermostat;

types::create_thermostat_dispatch!(Thermostat, dispatch_thermostat_request);

impl Thermostat {
    async fn sensor_reading(self, req: SensorReading) -> TargetTemp {
        println!("sensor reading! {:?}", req);
        TargetTemp(70.0)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("server started");
    let server_addr: SocketAddr = "0.0.0.0:20542".parse()?;
    let (server, _server_certs) = make_server_endpoint(server_addr)?;
    let channel = quic_rpc::transport::quinn::QuinnServerEndpoint::new(server)?;
    let target = Thermostat;
    run_server_loop(
        ThermostatService,
        channel.clone(),
        target,
        dispatch_thermostat_request,
    )
    .await?;
    Ok(())
}

fn make_server_endpoint(bind_addr: SocketAddr) -> anyhow::Result<(Endpoint, Vec<u8>)> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

fn configure_server() -> anyhow::Result<(ServerConfig, Vec<u8>)> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
    Arc::get_mut(&mut server_config.transport)
        .unwrap()
        .max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}
