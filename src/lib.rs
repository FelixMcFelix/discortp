//! DiscoRTP is a lightweight, flexible [Real-time Transport Protocol](https://tools.ietf.org/html/rfc3550)
//! parsing library designed for use in non-standards compliant environments, such as [Discord](discord.gg).
//!
//! DiscoRTP was originally developed for use in [Serenity](https://github.com/serenity-rs/serenity).

#[cfg(feature = "discord")]
#[cfg_attr(docsrs, doc(cfg(feature = "discord")))]
pub mod discord;

pub mod rtcp;
pub mod rtp;