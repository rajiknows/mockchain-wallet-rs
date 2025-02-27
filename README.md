# Mockchain Wallet CLI

[![Crates.io](https://img.shields.io/crates/v/mockchain-wallet-rs)](https://crates.io/crates/mockchain-wallet-rs)
[![Rust](https://img.shields.io/badge/rust-1.54.0%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/workflow/status/0xsouravm/mockchain-wallet-rs/CI)](https://github.com/0xsouravm/mockchain-wallet-rs/actions)
[![Docs](https://img.shields.io/docsrs/mockchain-wallet-rs)](https://docs.rs/mockchain-wallet-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A command-line interface for interacting with a [mockchain](https://github.com/0xsouravm/mockchain) network using a modular Rust architecture.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
  - [Create a Wallet](#create-a-wallet)
  - [List Wallets](#list-wallets)
  - [Check Balance](#check-balance)
  - [Send Transaction](#send-transaction)
  - [Request from Faucet](#request-from-faucet)
- [Architecture](#architecture)
- [Project Documentation](#project-documentation)
- [Development](#development)
  - [Building](#building)
  - [Adding New Features](#adding-new-features)
- [Security](#security)
- [Contributing](#contributing)
- [License](#license)

## Features

- ✅ Create and manage multiple wallets
- ✅ Check wallet balances
- ✅ Send transactions between wallets
- ✅ Request funds from mockchain faucet
- ✅ Secure local key storage
- ✅ Modular, maintainable codebase
- ✅ Comprehensive error handling

## Installation

### Prerequisites

- Rust and Cargo (1.54.0+)
- Running mockchain service at `http://[::1]:50051`

### From Source

```bash
# Clone repository
git clone https://github.com/0xsouravm/mockchain-wallet-rs.git
cd mockchain-wallet-rs

# Build release version
cargo build --release

# Run the executable
./target/release/mockallet
```

### From Crates.io

```bash
# Install Directly
cargo install mockchain-wallet-rs
```

## Quick Start

```bash
# Create a new wallet
./mockallet new alice

# Request funds from faucet
./mockallet faucet alice

# Check balance
./mockallet balance alice

# Create another wallet
./mockallet new bob

# Send transaction
./mockallet send alice bob 100
```

## Usage

### Create a Wallet

```bash
mockallet new <wallet_name>
```

Creates a new wallet with a randomly generated key pair.

### List Wallets

```bash
mockallet list
```

Displays all wallets in your local storage.

### Check Balance

```bash
mockallet balance <wallet_name>
```

Retrieves the current balance for a wallet.

### Send Transaction

```bash
mockallet send <from_wallet> <to_wallet> <amount>
```

Sends funds from one wallet to another. The recipient can be specified either by wallet name or by public key address.

### Request from Faucet

```bash
mockallet faucet <wallet_name>
```

Requests funds from the mockchain's faucet service.

## Architecture

This application follows a modular architecture for improved maintainability:

```
src/
├── main.rs         # Entry point with error handling
├── commands.rs     # Command definitions using StructOpt
├── models.rs       # Data structures
├── wallet.rs       # Mockchain interactions
├── storage.rs      # Wallet storage management
├── errors.rs       # Error handling system
└── proto.rs        # gRPC protocol initialisation
```

Key architectural decisions:
- Separation of concerns between data, storage, and network operations
- Custom error types with context-rich error messages
- Domain-driven design with clear boundaries between modules
- gRPC communication with the mockchain service

## Project Documentation

Generate and view the API documentation:

```bash
cargo doc --open
```

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Adding New Features

To add a new command:
1. Add a new variant to the `Command` enum in `commands.rs`
2. Add the command handler in the `run()` function in `main.rs`
3. Implement the necessary functionality in appropriate modules

## Security

- Private keys are stored locally in `.wallets/wallets.json`
- Keys use secp256k1 cryptography (same as Bitcoin)
- Transactions are signed with ECDSA signatures
- Private keys never leave your local machine

⚠️ **Warning**: Secure access to the `.wallets` directory on your machine


## Contributing

Contributions are welcome! Here are some areas where help would be appreciated:

- Improving the security with which the wallet keypairs are stored
- Including BIP39 mnemonics for wallet creation and recovery
- Adding password confirmation before sending transactions

Please feel free to submit a Pull Request.

Please follow the [Rust code style guidelines](https://rust-lang.github.io/api-guidelines/) and include appropriate tests.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/0xsouravm/mockchain-wallet-rs/blob/master/LICENSE) file for details.