use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6, UdpSocket};
use std::time::Duration;

use iroh::endpoint::presets::Minimal;
use iroh::endpoint::IncomingAddr;
use iroh::{Endpoint, EndpointAddr};
use tracing_subscriber::EnvFilter;

const ALPN: &[u8] = b"repro/ipv6-scope";
const PEER: Ipv6Addr = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);

fn v6_port(addrs: &[SocketAddr]) -> u16 {
    addrs.iter().find(|a| a.is_ipv6()).expect("ipv6 socket").port()
}

#[tokio::main]
async fn main() {
    let scope: u32 = std::env::args()
        .nth(1)
        .expect("usage: repro <ifindex>")
        .parse()
        .expect("ifindex");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("iroh::socket::transports=debug"))
        .with_writer(std::io::stdout)
        .without_time()
        .init();

    let probe = UdpSocket::bind("[::]:0").unwrap();
    let tx = UdpSocket::bind("[::]:0").unwrap();
    let dialed = SocketAddrV6::new(PEER, probe.local_addr().unwrap().port(), 0, scope);
    tx.send_to(b"x", dialed).unwrap();
    let (_, from) = probe.recv_from(&mut [0u8; 8]).unwrap();
    println!("std   dialed:                {dialed}");
    println!("std   recv_from reports:     {from}");

    let server = Endpoint::builder(Minimal)
        .alpns(vec![ALPN.to_vec()])
        .bind()
        .await
        .unwrap();
    let client = Endpoint::builder(Minimal).bind().await.unwrap();
    let dialed = SocketAddrV6::new(PEER, v6_port(&server.bound_sockets()), 0, scope);
    println!("iroh  dialed:                {dialed}");

    let accept = {
        let server = server.clone();
        tokio::spawn(async move {
            let incoming = server.accept().await.expect("incoming");
            let remote = match incoming.remote_addr() {
                IncomingAddr::Ip(addr) => addr.to_string(),
                other => format!("{other:?}"),
            };
            println!("iroh  Incoming::remote_addr: {remote}");
            incoming.accept().unwrap().await.map(|_| ())
        })
    };

    let addr = EndpointAddr::new(server.id()).with_ip_addr(dialed.into());
    let connect = tokio::time::timeout(Duration::from_secs(10), client.connect(addr, ALPN)).await;
    let outcome = match &connect {
        Ok(Ok(_)) => "Ok".to_string(),
        Ok(Err(e)) => format!("Err({e})"),
        Err(_) => "timed out".to_string(),
    };
    println!("iroh  connect (10s timeout): {outcome}");
    if matches!(connect, Ok(Ok(_))) {
        let handshake = accept.await.unwrap().map_err(|e| e.to_string());
        println!("iroh  server handshake:      {handshake:?}");
    }
    std::process::exit(0);
}
