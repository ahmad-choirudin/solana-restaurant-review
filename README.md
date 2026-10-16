Solana Restaurant Review

Overview

Solana Restaurant Review is a Solana program written in Rust for storing restaurant reviews on-chain.

Each review includes a title, location, rating, and description.

Features

- Add a restaurant review.
- Update an existing review.
- Store the restaurant's location alongside other review data.
- Serialize instructions and account state using Borsh.

Review Fields

- "title": Restaurant name or review title.
- "location": Restaurant location.
- "rating": Numeric rating from the client.
- "description": Written review of the restaurant.

Requirements

- Rust and Cargo
- Solana development tools
- A Solana wallet and test SOL for testnet deployment

Build and Test

Run these commands from the project directory:

cargo check
cargo test

A successful build does not replace integration testing. Test both Add and Update instructions before deployment.

Deployment Guide

1. Configure the Solana CLI to use Devnet.
2. Build the program using the SBF build tool supported by your Solana installation.
3. Deploy the compiled program to Devnet.
4. Record the deployed program ID.
5. Use a client that serializes "ReviewInstruction" with Borsh to interact with the program.

Account Requirements

The review account must be created and allocated before calling the program. It must be owned by this program, writable, and large enough to store the serialized review data.

Limitations

This project is a learning starter and has not been audited. It does not include a frontend, account-creation instruction, or integration tests. Additional validation and testing are needed before production use.

License

MIT
