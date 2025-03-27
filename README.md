# PVM Smart Contract - HackaRust

## Overview
This project is a smart contract written in Rust using `ink!` for Polkadot's Parachain Virtual Machine (PVM). It allows users to deposit funds and check their balances within the contract. This project was developed exclusively for a hackathon.

## Features
- Deposit function to store balance per user
- Retrieve balance for a specific account
- Unit tests to ensure contract reliability

## Setup Instructions

### Prerequisites
Ensure you have Rust installed and the `wasm32` target set up:
```sh
rustup update
rustup target add wasm32-unknown-unknown
```

Install `cargo-contract`:
```sh
cargo install cargo-contract --force
```

### Clone the Repository
```sh
git clone git@github.com:luizhenridev/hackaRust.git
cd hackaRust
```

### Build the Contract
```sh
cargo contract build
```

### Run a Local Substrate Node
```sh
cargo install substrate-contracts-node --force
substrate-contracts-node --dev
```

### Deploy the Contract
1. Open [Polkadot JS Apps](https://polkadot.js.org/apps/#/contracts)
2. Connect to your local Substrate node
3. Navigate to the **Contracts** tab
4. Click **Upload & Deploy Contract**
5. Select the `.wasm` file generated in `target` and deploy it

## Usage
After deploying, interact with the contract using Polkadot JS or through your own frontend.

## License
This project is open
