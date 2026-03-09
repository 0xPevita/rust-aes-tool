# rust-aes-tool

> AES-256-GCM file encryption CLI in Rust - same algorithm as BlobSafe.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)

## Overview

Rust reference implementation of the AES-256-GCM encryption used in [BlobSafe](https://github.com/0xPevita/blobsafe). Encrypt files from CLI before uploading to Shelby.

## Build
```bash
cargo build --release
```

## Usage
```bash
# Generate key
./aes-tool keygen

# Encrypt
./aes-tool encrypt --input secret.pdf --output secret.enc --key YOUR_KEY

# Decrypt
./aes-tool decrypt --input secret.enc --output secret.pdf --key YOUR_KEY
```

## Format

Output: `[12-byte nonce][AES-256-GCM ciphertext]` - identical to BlobSafe browser encryption for cross-platform compatibility.

## License
MIT
