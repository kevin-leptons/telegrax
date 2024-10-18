//! Send messages as a Telegram bot.
//!
//! The library focuses on sending messages for now, at least in version 1.
//! Later, other functions may be added if needed.
//!
//! # Example
//!
//! ```no_run
#![doc = include_str!("../examples/send_message.rs")]
//!```

pub mod bot;
pub mod limit;
pub mod result;
