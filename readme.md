# Telegrax

Send messages as a Telegram bot.

The library focuses on sending messages for now, at least in version 1. Later,
other functions may be added if needed.

## Quickstart

For a quickstart, see [examples](examples). For installation from package
repository, see [crates.io](https://crates.io/crates/telegrax). For API
references, see [doc.rs](https://docs.rs/telegrax).

## Test

```bash
# Create a proper configuration.
cp config.test.example.json config.test.json
vim config.test.json
chmod 600 config.test.json

# Run all tests.
cargo test

# Validate lints.
rustup component add clippy
cargo clippy
```
