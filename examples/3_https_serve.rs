//! Similar to 2_http_request.rs but this time, we serve the server with RFC7250 HTTPS instead of HTTP.
//! 
//! Instead of using a traditional TLS certificate with a Certificate Authority (CA),
//! we use the domain name public key to encrypt the TLS connection.
//! 
//!
//! Run with:
//!
//! ```bash
//! cargo run --example 3_https_serve
//! ```
//! 
//! Important: RFC7250 HTTPS is not yet supported by browsers. Pkarr can guarantee browser compatibility with a ICANN fallback though.

use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use pkarr::{Client, Keypair, PublicKey, SignedPacket, dns::rdata::SVCB};
use std::{error::Error, net::{Ipv4Addr, SocketAddr}, sync::Arc};

/// Publish the packet to the network
async fn publish_packet(client: &Client, server_port: u16, keypair: &Keypair) -> Result<PublicKey, Box<dyn Error>> {
    let target_ip = Ipv4Addr::new(127, 0, 0, 1);

    // The reqwest client only supports SVCB records for now
    let mut svcb = SVCB::new(0, ".".try_into().unwrap());
    svcb.set_port(server_port);
    let signed_packet = SignedPacket::builder()
        .https(".".try_into().unwrap(), svcb, 300)
        .address(
            ".".try_into().unwrap(),
            std::net::IpAddr::V4(target_ip),
            300,
        )
        .sign(&keypair)?;

    client.publish(&signed_packet, None).await?;
    Ok(keypair.public_key())
}

/// Starts a simple server on port 55632
async fn start_server(keypair: &Keypair) -> Result<SocketAddr, Box<dyn Error>> {
    let app = Router::new().route("/", get(|| async { "Hello world" }));

    let addr = "127.0.0.1:55632".parse::<SocketAddr>().unwrap();

    let server = axum_server::bind_rustls(
        addr,
        RustlsConfig::from_config(Arc::new(keypair.to_rpk_rustls_server_config())),
    );

    println!("Server running on https://{}", addr);

    tokio::spawn(async move {
        server.serve(app.into_make_service()).await.expect("Failed to start server");
    });
    Ok(addr)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let keypair = Keypair::random();
    // Start the server
    let socket_addr = start_server(&keypair).await.unwrap();

    // Publish the pkarr zone
    let pkarr_client = Client::builder().build()?;
    let public_key = publish_packet(&pkarr_client, socket_addr.port(), &keypair).await?;
    println!("Published packet: {}", public_key);
    println!();

    // HTTPS call to the server
    let http_client = reqwest::ClientBuilder::from(pkarr_client).build()?;
    let url = format!("https://{}/", public_key);
    println!("Requesting URL: {}", url);
    let response = http_client.get(url).send().await?;

    println!("Response HTTPS");
    println!("Status {:?}", response.status());
    println!("Body: {}", response.text().await?);

    Ok(())
}
