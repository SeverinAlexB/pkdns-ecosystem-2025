//! Publish a pkarr packet to the network
//!
//! This example shows how to publish a pkarr packet to the network.
//! It generates a new key pair and publishes a packet with an A record.
//! It then retrieves the packet and prints the answers.
//! 
//! Important: The packet needs to be republished every 1 hour to keep it alive.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example 1_publish_dns_zone
//! ```

use pkarr::{dns::Name, Client, Keypair, SignedPacket};
use std::{error::Error, net::Ipv4Addr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a pkarr client
    let client = Client::builder().build()?;

    // Generate a new key pair (or you can use an existing one)
    let keypair = Keypair::random();
    println!("Generated keypair: {}", keypair.public_key());

    let packet = SignedPacket::builder()
        .a(Name::new(".")?, Ipv4Addr::new(127, 0, 0, 1), 300)
        .txt(Name::new(".")?, "Hello, world!".try_into()?, 300)
        .build(&keypair)?;

    // Publish the packet
    println!("Publishing pkarr packet...");
    client.publish(&packet, None).await?;

    println!("Packet published successfully!\n");

    // You can also retrieve the packet to verify it was published
    println!("Retrieving the published packet...");
    let retrieved_packet = client.resolve(&keypair.public_key()).await.ok_or("Packet not found")?;

    println!("Retrieved packet at {}", retrieved_packet.timestamp().format_http_date());
    for answer in retrieved_packet.all_resource_records() {
        println!("- {:?}", answer);
    }

    Ok(())
}
