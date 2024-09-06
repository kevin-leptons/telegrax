# Telegrax

Telegram Application Programming Interfaces (APIs). At the moment, there is only
API for sending message from a bot to a chat.

## Quickstart

For a quickstart, see [examples](examples). For installation from package
repository, see [crates.io](https://crates.io/crates/telegrax). For API
references, see [doc.rs](https://docs.rs/telegrax).

## Test

```bash
# Create a proper configuration.
cp config.test.example.json config.test.json
vim config.test.json

# Run all tests.
cargo test
```
