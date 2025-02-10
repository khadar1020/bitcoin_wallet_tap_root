# Bitcoin Wallet Generator

This project is a simple Bitcoin wallet generator that creates two types of Bitcoin addresses: **Pay to Witness Public Key Hash (P2WPKH)** and **Pay to Taproot (P2TR)**. The wallet generator uses the `bitcoin` and `rand` crates in Rust to generate key pairs and addresses.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Dependencies](#dependencies)
- [Installation](#installation)
- [Usage](#usage)
- [Flowchart](#flowchart)
- [Contributing](#contributing)
- [License](#license)

## Introduction
This project demonstrates how to generate Bitcoin addresses using Rust. It leverages the `bitcoin` crate to handle cryptographic operations and address generation. The generated addresses are printed along with their **QR code URIs**, which can be used to receive Bitcoin payments.

## Features
- **P2WPKH Address Generation**: Generates a **Pay to Witness Public Key Hash (P2WPKH)** address.
- **P2TR Address Generation**: Generates a **Pay to Taproot (P2TR)** address.
- **QR Code URIs**: Outputs QR code URIs for easy sharing and scanning.

## Dependencies
This project requires the following Rust libraries:
- [`bitcoin`](https://crates.io/crates/bitcoin) - A Rust library for Bitcoin protocol.
- [`rand`](https://crates.io/crates/rand) - A Rust library for random number generation.

## Installation

### Clone the Repository
```bash
git clone https://github.com/yourusername/bitcoin-wallet-generator.git
cd bitcoin-wallet-generator
```

### Build the Project
```bash
cargo build --release
```

### Run the Project
```bash
cargo run --release
```

## Flowchart

_Add a flowchart image here if applicable._

## Contributing
Contributions are welcome! Please follow these steps to contribute:
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m "Add new feature"`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a Pull Request.

## License
This project is licensed under the [MIT License](LICENSE).
