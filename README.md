# Toy Blockchain

A simple blockchain implementation in Rust featuring proof-of-work mining and SHA-256 hashing.

## Features

- **Proof-of-Work Mining**: Implements a proof-of-work consensus mechanism with configurable difficulty
- **SHA-256 Hashing**: Uses SHA-256 cryptographic hashing for block integrity
- **Parallel Mining**: Leverages Rayon for parallel nonce searching to speed up mining
- **Chain Validation**: Built-in validation to ensure blockchain integrity

## Project Structure

- `src/block.rs` - Block structure and mining implementation
- `src/blockchain.rs` - Blockchain structure and chain operations
- `src/lib.rs` - Library exports
- `src/main.rs` - Example usage

## Usage

Run the example:

```bash
cargo run
```

Use as a library:

```rust
use toy_blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First block data".to_string());
    blockchain.add_block("Second block data".to_string());

    println!("Is valid: {}", blockchain.is_chain_valid());
    println!("{}", blockchain);
}
```

## How It Works

1. **Block Creation**: Each block contains data, a timestamp, a nonce, the previous block's hash, and its own hash
2. **Mining**: The mining process searches for a nonce that produces a hash starting with a specified number of zeros (difficulty)
3. **Chain Validation**: The blockchain validates that each block's hash is correct and matches the previous block's hash reference

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Dependencies

- `sha2` - SHA-256 hashing
- `bytes` - Efficient byte buffer handling
- `rayon` - Data parallelism for mining
- `hex` - Hexadecimal encoding/decoding

## License

MIT
