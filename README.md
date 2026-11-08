# Solana Restaurant Review

A Solana program (Rust, native `solana-program`) that stores restaurant reviews on-chain. Instructions and account state use Borsh.

## Features

- `Add` a review (title, location, rating, description).
- `Update` an existing review. Only the wallet that added it can update it.
- Validation: rating must be 1 to 5; title and location up to 100 bytes; description up to 1000 bytes.

## Instruction accounts

| # | Account | Requirements |
| --- | --- | --- |
| 0 | Review author | Signer |
| 1 | Review account | Writable, owned by this program, allocated with at least `AccountState::MAX_SIZE` bytes (1246) |

## Account state

`is_initialized`, `initializer` (author's public key, 32 bytes), `title`, `location`, `rating`, `description`.

A freshly allocated zero-filled account decodes as not initialized, so `Add` works once and `Update` is rejected until then.

## Custom errors

| Code | Meaning |
| --- | --- |
| 0 | Invalid rating (not 1 to 5) |
| 1 | Title too long |
| 2 | Location too long |
| 3 | Description too long |
| 4 | Unauthorized (not the original author) |

## Build and test

```bash
cargo check
cargo test
```

Build for deployment with your Solana CLI's SBF tool (for example `cargo build-sbf`), deploy to Devnet, and record the program ID. Unit tests do not replace integration tests on a real cluster.

## Limitations

- There is no account-creation instruction; the client must create and allocate the review account first. A PDA derived from the author and title would be a better design.
- Not audited. Use test networks only.

## License

MIT
