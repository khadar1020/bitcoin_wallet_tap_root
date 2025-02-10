pub mod bitcoin_client;

fn main() {
    if let Err(e) = bitcoin_client::generate_wallet() {
        eprintln!("Error generating wallet: {}", e);
        std::process::exit(1);
    }
    println!("Wallet generated successfully!");
}