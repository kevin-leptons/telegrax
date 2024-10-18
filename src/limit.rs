//! Endpoint's rate limits.
//!
//! These limits are relative. A limit may go a little bit higher but eventually
//! results in error [Error::RateLimit]. For more details, see [Broadcasting to
//! users](https://core.telegram.org/bots/faq#broadcasting-to-users).

/// How many messages are send to a chat per second.
pub const MESSAGE_PER_CHAT_PER_SECOND: usize = 1;

/// How many messages are send to a chat per a minute.
pub const MESSAGE_PER_CHAT_PER_MINUTE: usize = 20;

/// How many messages are send per second.
pub const MESSAGE_PER_SECOND: usize = 30;
