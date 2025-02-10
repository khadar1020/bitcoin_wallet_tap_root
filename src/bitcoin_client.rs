use bitcoin::{
    secp256k1::{Secp256k1, Keypair},
    Address, Network, PublicKey, XOnlyPublicKey
};
use rand::thread_rng;

pub fn generate_wallet() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize secp256k1 context
    let secp = Secp256k1::new();
    let network = Network::Testnet;

    // Generate keypair
    let (secret_key, public_key) = secp.generate_keypair(&mut thread_rng());
    
    // Create key pair for Taproot
    let key_pair = Keypair::from_secret_key(&secp, &secret_key);
    let (x_only_public_key, _parity) = XOnlyPublicKey::from_keypair(&key_pair);

    // Generate P2TR (Pay to Taproot) address
    let tap_address = Address::p2tr(&secp, x_only_public_key, None, network);

    // Generate P2WPKH (Pay to Witness Public Key Hash) address
    let public_key = PublicKey::new(public_key);
    let p2wpkh_address = Address::p2wpkh(&public_key, network)?;

    // Print addresses with QR URIs
    println!("Pay to Witness Public Key Hash (P2WPKH): {}", 
             p2wpkh_address.to_qr_uri().to_ascii_lowercase());
    println!("Pay to Taproot (P2TR): {}", 
             tap_address.to_qr_uri().to_ascii_lowercase());

    Ok(())
}