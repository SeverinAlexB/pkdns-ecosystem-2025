//! Make an HTTP request to a server that is behind a pkdns domain
//!
//! This example shows how to make an HTTP request to a server that is behind a pkdns domain.
//! It publishes a packet with an SVCB record and an A record, and then makes an HTTP request to the server.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example 2_http_request
//! ```

use axum::{Router, routing::get};
use pkarr::{Client, Keypair, PublicKey, SignedPacket, dns::rdata::SVCB};
use std::{error::Error, net::{Ipv4Addr, SocketAddr}};

/// Publish a example packet
async fn publish_packet(client: &Client, server_port: u16) -> Result<PublicKey, Box<dyn Error>> {
    let keypair = Keypair::random();

    // The reqwest client only supports SVCB records for now
    let mut svcb = SVCB::new(0, ".".try_into().unwrap());
    svcb.set_port(server_port);
    let signed_packet = SignedPacket::builder()
        .https("".try_into().unwrap(), svcb, 300)
        .address(
            ".".try_into().unwrap(),
            std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            300,
        )
        .sign(&keypair)?;

    client.publish(&signed_packet, None).await?;
    Ok(keypair.public_key())
}

/// Starts a simple server on a random port
async fn start_server() -> Result<SocketAddr, Box<dyn Error>> {
    let app = Router::new().route("/", get(|| async { "Hello world" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
    let addr = listener.local_addr().unwrap();
    println!("Server running on http://{}", addr);

    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("Failed to start server");
    });
    Ok(addr)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start the server
    let socket_addr = start_server().await.unwrap();

    // Publish the pkarr zone
    let pkarr_client = Client::builder().build()?;
    let public_key = publish_packet(&pkarr_client, socket_addr.port()).await?;
    println!("Published packet: {}", public_key);

    // HTTP call to the server
    let http_client = reqwest::ClientBuilder::from(pkarr_client).build()?;
    let url = format!("http://{}/", public_key);
    println!("Requesting URL: {}", url);
    let response = http_client.get(url).send().await?;
    println!();
    println!("Response");
    println!("Status {:?}", response.status());
    println!("Body: {}", response.text().await?);

    Ok(())
}
