//! Readers and writers for the [Real-time Transport Protocol].
//!
//! *These are included when using the `"rtp"` feature.*
//!
//! [Real-time Transport Protocol]: https://tools.ietf.org/html/rfc3550

use crate::wrap::*;
use pnet_macros_support::{
	packet::PrimitiveValues,
	types::*,
};

include!(concat!(env!("OUT_DIR"), "/rtp.rs"));

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
