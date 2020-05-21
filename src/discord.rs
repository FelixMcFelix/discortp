//! Additional packet formats used by Discord.
//!
//! *These are included when using the `"discord"` feature.*

use pnet_macros_support::{
	packet::PrimitiveValues,
	types::*,
};
use std::mem;

include!(concat!(env!("OUT_DIR"), "/discord.rs"));

const FIXED_SIZE_COMPONENT: usize =
	std::mem::size_of::<u16>() +
	std::mem::size_of::<u32>();

const IP_DISCOVERY_LEN: usize = IpDiscoveryPacket::minimum_packet_size() + 64;

impl IpDiscoveryPacket<'_> {
	/// Standard packet length when using Discord-specified lengths.
	pub const fn const_packet_size() -> usize {
		IP_DISCOVERY_LEN
	}
}

impl MutableIpDiscoveryPacket<'_> {
	/// Standard packet length when using Discord-specified lengths.
	pub const fn const_packet_size() -> usize {
		IP_DISCOVERY_LEN
	}
}

/// Packet type for Discord's IP Discovery.
///
/// [`Other`] values are illegal.
///
/// [`Other`]: #variant.Other
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IpDiscoveryType {
	Request,
	Response,
	Other(u16),
}

impl IpDiscoveryType {
	fn new(val: u16) -> Self {
		match val {
			1 => Self::Request,
			2 => Self::Response,
			_ => Self::Other(val),
		}
	}
}

impl PrimitiveValues for IpDiscoveryType {
	type T = (u16,);

	fn to_primitive_values(&self) -> Self::T {
		match self {
			Self::Request => (1,),
			Self::Response => (2,),
			Self::Other(n) => (*n,),
		}
	}
}