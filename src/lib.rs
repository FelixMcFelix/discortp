//! DiscoRTP is a lightweight, flexible [Real-time Transport Protocol]
//! parsing library designed for use in non-standards compliant environments, such as [Discord].
//!
//! DiscoRTP differs from other Rust RTP libraries in that packet construction *should
//! never fail*, unless there are too few bytes. Not all implementations treat fields as they
//! should (*i.e.*, length), and so DiscoRTP's philosophy is that **the user knows best**.
//! Packet parsers are building blocks to be manually assembled, and validation mechanisms
//! exist but are manual.
//!
//! DiscoRTP was originally developed for use in [Serenity], and is built using [pnet].
//!
//! All crate features are optional:
//! * `"rtp"` includes copy-free and owned views of RTP packets. *Default*.
//! * `"rtcp"` includes copy-free and owned views of RTCP packets. *Default*.
//! * `"pnet"` re-includes traits from [pnet] for packet view manipulation. *Default*.
//! * `"demux"` includes utilities for separating multiplexed RTP/RTCP streams.
//! * `"discord"` includes platform-specific packet formats for Discord.
//!
//! [Real-time Transport Protocol]: https://tools.ietf.org/html/rfc3550
//! [Discord]: https://discord.gg
//! [Serenity]: https://github.com/serenity-rs/serenity
//! [pnet]: https://docs.rs/pnet

#[cfg(feature = "demux")]
pub mod demux;

#[cfg(feature = "discord")]
pub mod discord;

#[cfg(feature = "rtcp")]
pub mod rtcp;

#[cfg(feature = "rtp")]
pub mod rtp;

#[cfg(feature = "pnet")]
pub use pnet_macros_support::{
	self as pnet,
	packet::{
		FromPacket,
		MutablePacket,
		Packet,
		PacketSize,
	},
};
