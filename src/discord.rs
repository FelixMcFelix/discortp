//! Additional packet formats used by Discord.
//!
//! *These are included when using the `"discord"` feature.*

use pnet_macros::packet;
use pnet_macros_support::{packet::PrimitiveValues, types::*};
use std::mem;

#[packet]
#[derive(Eq, PartialEq)]
/// Packet format for Discord's [IP Discovery], used in NAT tunnelling.
///
/// A description of fields:
///
/// ## [`pkt_type`]
/// Denotes whether this packet is a request or response.
///
/// ## [`length`]
/// Length (in bytes) of all successive fields.
/// This controls the string length of [`address`].
///
/// In ordinary use, this should be set to 70.
///
/// ## [`ssrc`]
/// SSRC that the requesting client has been assigned to use over RTP.
///
/// ## [`address`]
/// Null-terminated C-string containing the address of the
/// requester as seen by the server.
///
/// Requests should leave this empty.
///
/// ## [`port`]
/// Client's source port, as seen by the server.
///
/// Requests may include their destination port.
///
/// ## [`payload`]
/// No payload exists for this packet type: 0-length.
///
/// [IP Discovery]: https://discordapp.com/developers/docs/topics/voice-connections#ip-discovery
/// [`pkt_type`]: #structfield.pkt_type
/// [`length`]: #structfield.length
/// [`ssrc`]: #structfield.ssrc
/// [`address`]: #structfield.address
/// [`port`]: #structfield.port
/// [`payload`]: #structfield.payload
pub struct IpDiscovery {
	#[construct_with(u16be)]
	pub pkt_type: IpDiscoveryType,

	pub length: u16be,

	pub ssrc: u32be,

	#[length = "length - FIXED_SIZE_COMPONENT"]
	pub address: Vec<u8>,

	pub port: u16be,

	#[payload]
	#[length = "0"]
	pub payload: Vec<u8>,
}

#[packet]
#[derive(Eq, PartialEq)]
/// Packet format for Discord's UDP Keepalives.
///
/// A description of fields:
///
/// ## [`ssrc`]
/// SSRC of the sending source.
///
/// ## [`payload`]
/// No payload exists for this packet type: 0-length.
///
/// [`ssrc`]: #structfield.ssrc
/// [`payload`]: #structfield.payload
pub struct Keepalive {
	pub ssrc: u32be,

	#[payload]
	#[length = "0"]
	pub payload: Vec<u8>,
}

const FIXED_SIZE_COMPONENT: usize = std::mem::size_of::<u16>() + std::mem::size_of::<u32>();

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
#[non_exhaustive]
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
