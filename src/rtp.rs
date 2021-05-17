//! Readers and writers for the [Real-time Transport Protocol].
//!
//! *These are included when using the `"rtp"` feature.*
//!
//! [Real-time Transport Protocol]: https://tools.ietf.org/html/rfc3550

use crate::wrap::*;
use pnet_macros::packet;
use pnet_macros_support::{packet::PrimitiveValues, types::*};

#[packet]
#[derive(Eq, PartialEq)]
/// Packet header for the [Real-time Transport Protocol].
///
/// A description of fields:
///
/// ## `version`
/// RTP version. Should be `2`.
///
/// ## `padding`
/// Packet contains padding octets which are not part of the payload.
/// The last byte of the payload contains the
/// count of bytes to be ignored from the end (including itself).
///
/// ## `extension`
/// If set, the fixed header must be followed by exactly one [`RtpExtension`].
///
/// ## `csrc_count`
/// Number of CSRC identifiers following the fixed header.
///
/// ## `marker`
/// Bitflag with application/profile-specific meaning, *e.g.*, frame boundaries.
///
/// ## `payload_type`
/// Describes the format of the RTP [`payload`].
///
/// ## `sequence`
/// Increments by `1` for every packet sent, and used to detect packet loss
/// or enable reordering.
///
/// Should be initialised to a random number.
///
/// ## `timestamp`
/// Sampling instant for the first octet of the payload: must be derived from a
/// monotonically and linearly increasing clock. If data are generated periodically,
/// then the measure should be derived from the sample clock (*e.g.*, increment by
/// one for every sampling period).
///
/// Should be initialised to a random number.
///
/// ## `ssrc`
/// Synchronisation Source, identifying the sender.
///
/// Should be initialised to a random number.
///
/// ## `csrc_list`
/// Used to identify contributing SSRCs in the packet (*i.e.*, to allow speaker
/// detection if packets are mixed and re-encoded along the path).
///
/// ## `payload`
/// Bytes of the RTP body.
///
/// [`payload`]: #structfield.payload
/// [`RtpExtension`]: struct.RtpExtension.html
/// [Real-time Transport Protocol]: https://tools.ietf.org/html/rfc3550
pub struct Rtp {
	pub version: u2,

	pub padding: u1,

	pub extension: u1,

	pub csrc_count: u4,

	pub marker: u1,

	#[construct_with(u7)]
	pub payload_type: RtpType,

	#[construct_with(u16be)]
	pub sequence: Wrap16,

	#[construct_with(u32be)]
	pub timestamp: Wrap32,

	pub ssrc: u32be,

	#[length = "csrc_count"]
	pub csrc_list: Vec<u32be>,

	#[payload]
	pub payload: Vec<u8>,
}

#[packet]
#[derive(Eq, PartialEq)]
/// Extension data for an [`Rtp`] packet.
///
/// A description of fields:
///
/// ## `info`
/// Profile-defined, usually an extension type.
///
/// ## `length`
/// Number of 32-bit words in `ext_data`. `0` is valid.
///
/// ## `ext_data`
/// Application-defined extension data.
///
/// ## `payload`
/// Remainder of packet data.
///
/// [`Rtp`]: struct.Rtp.html
pub struct RtpExtension {
	pub info: u16be,

	pub length: u16be,

	#[length = "4 * length"]
	pub ext_data: Vec<u8>,

	#[payload]
	pub payload: Vec<u8>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
/// RTP message types. These define the packet format used for the payload.
///
/// See the [IANA page] on the matter for an up-to-date-list.
///
/// [IANA page]: https://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-1
pub enum RtpType {
	/// Code 0.
	Pcmu,

	/// Code 3.
	Gsm,

	/// Code 4.
	G723,

	/// Code 5--6, 16--17.
	Dvi4(u8),

	/// Code 7.
	Lpc,

	/// Code 8.
	Pcma,

	/// Code 9.
	G722,

	/// Code 10.
	L16Stereo,

	/// Code 11.
	L16Mono,

	/// Code 12.
	Qcelp,

	/// Code 13.
	Cn,

	/// Code 14.
	Mpa,

	/// Code 15.
	G728,

	/// Code 18.
	G729,

	/// Code 25.
	CelB,

	/// Code 26.
	Jpeg,

	/// Code 28.
	Nv,

	/// Code 31.
	H261,

	/// Code 32.
	Mpv,

	/// Code 33.
	Mp2t,

	/// Code 34.
	H263,

	/// Dynamically assigned payload type (codes >= 96).
	Dynamic(u8),

	/// Reserved payload types, typically to mitigate RTCP packet type collisions (1--2, 19, 72--76).
	Reserved(u8),

	/// Unassigned payload type (all remaining < 128).
	Unassigned(u8),

	/// Code point too high for u7: application error?
	Illegal(u8),
}

impl RtpType {
	pub fn new(val: u7) -> Self {
		use RtpType::*;
		match val {
			0 => Pcmu,
			3 => Gsm,
			4 => G723,
			5 | 6 | 16 | 17 => Dvi4(val),
			7 => Lpc,
			8 => Pcma,
			9 => G722,
			10 => L16Stereo,
			11 => L16Mono,
			12 => Qcelp,
			13 => Cn,
			14 => Mpa,
			15 => G728,
			18 => G729,
			25 => CelB,
			26 => Jpeg,
			28 => Nv,
			31 => H261,
			32 => Mpv,
			33 => Mp2t,
			34 => H263,
			1..=2 | 19 | 72..=76 => Reserved(val),
			96..=127 => Dynamic(val),
			128..=255 => Illegal(val),
			_ => Unassigned(val),
		}
	}
}

impl PrimitiveValues for RtpType {
	type T = (u7,);

	fn to_primitive_values(&self) -> Self::T {
		use RtpType::*;
		match self {
			Pcmu => (0,),
			Gsm => (3,),
			G723 => (4,),
			Dvi4(val) => (*val,),
			Lpc => (7,),
			Pcma => (8,),
			G722 => (9,),
			L16Stereo => (10,),
			L16Mono => (11,),
			Qcelp => (12,),
			Cn => (13,),
			Mpa => (14,),
			G728 => (15,),
			G729 => (18,),
			CelB => (25,),
			Jpeg => (26,),
			Nv => (28,),
			H261 => (31,),
			Mpv => (32,),
			Mp2t => (33,),
			H263 => (34,),

			Dynamic(val) => (*val,),
			Unassigned(val) => (*val,),
			Reserved(val) => (*val,),
			Illegal(val) => (*val,),
		}
	}
}
