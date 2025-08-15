//! Generate a keypair, save it to a file, and load it back.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example 4_save_and_load_key
//! ```

use pkarr::Keypair;
use std::error::Error;
use std::fs;
use std::path::Path;

const KEY_FILE: &str = "secret.hex";

fn save_keypair(keypair: &Keypair) -> Result<(), Box<dyn Error>> {
    let hex = hex::encode(keypair.secret_key());
    fs::write(KEY_FILE, hex)?;
    println!("Key saved to {}", KEY_FILE);
    Ok(())
}

fn load_keypair() -> Result<Keypair, Box<dyn Error>> {
    if !Path::new(KEY_FILE).exists() {
        return Err(format!("Key file {} not found", KEY_FILE).into());
    }
    
    let hex = fs::read_to_string(KEY_FILE)?.trim().to_string();
    let secret_key_bytes = hex::decode(hex)?;
    
    let secret_key: [u8; 32] = secret_key_bytes.try_into().map_err(|_| "Invalid secret key length")?;
    let keypair = Keypair::from_secret_key(&secret_key);
    Ok(keypair)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Generating new keypair");
    let keypair = Keypair::random();
    println!("Generated keypair: {}", keypair.public_key());
    
    // Save the keypair
    save_keypair(&keypair)?;

    let keypair = load_keypair()?;
    println!("Loaded keypair: {}", keypair.public_key());

    Ok(())
}
