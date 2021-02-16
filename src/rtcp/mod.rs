//! Readers and writers for the [RTP Control Protocol](https://tools.ietf.org/html/rfc3550#section-6).
//!
//! *These are included when using the `"rtcp"` feature.*

pub mod report;
use crate::{FromPacket, MutablePacket, Packet, PacketSize};
use pnet_macros_support::packet::PrimitiveValues;
use report::*;

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Rtcp {
	SenderReport(SenderReport),
	ReceiverReport(ReceiverReport),

	KnownType(RtcpType),
}

/// RTCP packet variants separated from the same stream.
#[derive(Debug)]
#[non_exhaustive]
pub enum RtcpPacket<'a> {
	SenderReport(SenderReportPacket<'a>),
	ReceiverReport(ReceiverReportPacket<'a>),

	KnownType(RtcpType),
}

impl RtcpPacket<'_> {
	pub fn new<'a>(pkt: &'a [u8]) -> Option<RtcpPacket<'a>> {
		RtcpType::from_packet(pkt).and_then(|rtcp_id| rtcp_id.decode(pkt))
	}
}

impl<'a> Packet for RtcpPacket<'a> {
	fn packet(&self) -> &[u8] {
		use RtcpPacket::*;

		match self {
			SenderReport(s) => s.packet(),
			ReceiverReport(s) => s.packet(),
			_ => unimplemented!(),
		}
	}

	fn payload(&self) -> &[u8] {
		use RtcpPacket::*;

		match self {
			SenderReport(s) => s.payload(),
			ReceiverReport(s) => s.payload(),
			_ => unimplemented!(),
		}
	}
}

impl<'a> FromPacket for RtcpPacket<'a> {
	type T = Rtcp;

	fn from_packet(&self) -> Self::T {
		use RtcpPacket::*;

		match self {
			SenderReport(s) => Rtcp::SenderReport(s.from_packet()),
			ReceiverReport(s) => Rtcp::ReceiverReport(s.from_packet()),
			_ => unimplemented!(),
		}
	}
}

impl<'a> PacketSize for RtcpPacket<'a> {
	fn packet_size(&self) -> usize {
		use RtcpPacket::*;

		match self {
			SenderReport(s) => s.packet_size(),
			ReceiverReport(s) => s.packet_size(),
			_ => unimplemented!(),
		}
	}
}

/// Mutable RTP/RTCP packets separated from the same stream.
#[derive(Debug)]
#[non_exhaustive]
pub enum MutableRtcpPacket<'a> {
	SenderReport(MutableSenderReportPacket<'a>),
	ReceiverReport(MutableReceiverReportPacket<'a>),
	
	KnownType(RtcpType),
}

impl MutableRtcpPacket<'_> {
	pub fn new<'a>(pkt: &'a mut [u8]) -> Option<MutableRtcpPacket<'a>> {
		RtcpType::from_packet(pkt).and_then(move |rtcp_id| rtcp_id.decode_mut(pkt))
	}
}

impl<'a> Packet for MutableRtcpPacket<'a> {
	fn packet(&self) -> &[u8] {
		use MutableRtcpPacket::*;

		match self {
			SenderReport(s) => s.packet(),
			ReceiverReport(s) => s.packet(),
			_ => unimplemented!(),
		}
	}

	fn payload(&self) -> &[u8] {
		use MutableRtcpPacket::*;

		match self {
			SenderReport(s) => s.payload(),
			ReceiverReport(s) => s.payload(),
			_ => unimplemented!(),
		}
	}
}

impl<'a> MutablePacket for MutableRtcpPacket<'a> {
	fn packet_mut(&mut self) -> &mut [u8] {
		use MutableRtcpPacket::*;

		match self {
			SenderReport(s) => s.packet_mut(),
			ReceiverReport(s) => s.packet_mut(),
			_ => unimplemented!(),
		}
	}

	fn payload_mut(&mut self) -> &mut [u8] {
		use MutableRtcpPacket::*;

		match self {
			SenderReport(s) => s.payload_mut(),
			ReceiverReport(s) => s.payload_mut(),
			_ => unimplemented!(),
		}
	}
}

impl<'a> FromPacket for MutableRtcpPacket<'a> {
	type T = Rtcp;

	fn from_packet(&self) -> Self::T {
		use MutableRtcpPacket::*;

		match self {
			SenderReport(s) => Rtcp::SenderReport(s.from_packet()),
			ReceiverReport(s) => Rtcp::ReceiverReport(s.from_packet()),
			_ => unimplemented!(),
		}
	}
}

impl<'a> PacketSize for MutableRtcpPacket<'a> {
	fn packet_size(&self) -> usize {
		use MutableRtcpPacket::*;

		match self {
			SenderReport(s) => s.packet_size(),
			ReceiverReport(s) => s.packet_size(),
			_ => unimplemented!(),
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
/// RTCP message types. These define the packet format used for both the header and payload.
///
/// See the [IANA page] on the matter for an up-to-date-list.
///
/// [IANA page]: https://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-4
pub enum RtcpType {
	/// SMPTE time-code mapping.
	///
	/// See [RFC 5484](https://tools.ietf.org/html/rfc5484).
	///
	/// Code 194.
	SmpteMap,

	/// Extended inter-arrival jitter report.
	///
	/// See [RFC 5450](https://tools.ietf.org/html/rfc5450).
	///
	/// Code 195.
	JitterReport,

	/// Sender report, containing jitter, reception, timing and volume information.
	///
	/// See the relevant [RTP RFC section](https://tools.ietf.org/html/rfc3550#section-6.4.1).
	/// Unlike the `ReceiverReport`, this includes a `SenderInfo` block.
	///
	/// Code 200.
	SenderReport,

	/// Sender report, containing jitter and reception information.
	///
	/// See the relevant [RTP RFC section](https://tools.ietf.org/html/rfc3550#section-6.4.2).
	///
	/// Code 201.
	ReceiverReport,

	/// Source description, mapping SSRC/CCRC values to information about each host.
	///
	/// See the relevant [RTP RFC section](https://tools.ietf.org/html/rfc3550#section-6.5).
	///
	/// Code 202.
	SourceDescription,

	/// Source exit message, denoting SSRC/CCRC of exiting hosts and an optional reason string.
	///
	/// See the relevant [RTP RFC section](https://tools.ietf.org/html/rfc3550#section-6.6).
	///
	/// Code 203.
	Goodbye,

	/// Application-defined RTCP message, containing a name and arbitrary data.
	///
	/// See the relevant [RTP RFC section](https://tools.ietf.org/html/rfc3550#section-6.7).
	///
	/// Code 204.
	ApplicationDefined,

	/// RTPFB, feedback on the RTP transport layer.
	///
	/// See [RFC 4585](https://tools.ietf.org/html/rfc4585)
	///
	/// Code 205.
	TransportFeedback,

	/// PSFB, feedback on the payload.
	///
	/// See [RFC 4585](https://tools.ietf.org/html/rfc4585)
	///
	/// Code 206.
	PayloadFeedback,

	/// Extended Report message, used for additional/mixed report blocks.
	///
	/// See [RTCP XR](https://tools.ietf.org/html/rfc3611).
	///
	/// Code 207.
	ExtendedReport,

	/// AVB RTCP packet.
	///
	/// See [IEEE P1733](https://ieeexplore.ieee.org/document/5154142).
	///
	/// Code 208.
	Avb,

	/// Receiver Summary information.
	///
	/// See [RFC 5760](https://tools.ietf.org/html/rfc5760).
	///
	/// Code 209.
	ReceiverSummary,

	/// Port mapping.
	///
	/// See [RFC 6284](https://tools.ietf.org/html/rfc6284).
	///
	/// Code 210.
	PortMapping,

	/// IDMS settings.
	///
	/// See [RFC 7272](https://tools.ietf.org/html/rfc7272).
	///
	/// Code 211.
	Idms,

	/// Reporting group reporting sources.
	///
	/// See the [draft RFC](https://datatracker.ietf.org/doc/draft-ietf-avtcore-rtp-multi-stream-optimisation/).
	///
	/// Code 212.
	ReportingGroupSources,

	/// Splicing notification message.
	///
	/// See [RFC 8286](https://tools.ietf.org/html/rfc8286).
	///
	/// Code 213.
	SplicingNotification,

	/// Explicitly reserved code point.
	Reserved(u8),

	/// Unknown message type.
	Unassigned(u8),
}

impl<'a> RtcpType {
	pub fn new(val: u8) -> Self {
		use RtcpType::*;
		match val {
			194 => SmpteMap,
			195 => JitterReport,
			200 => SenderReport,
			201 => ReceiverReport,
			202 => SourceDescription,
			203 => Goodbye,
			204 => ApplicationDefined,
			205 => TransportFeedback,
			206 => PayloadFeedback,
			207 => ExtendedReport,
			208 => Avb,
			209 => ReceiverSummary,
			210 => PortMapping,
			211 => Idms,
			212 => ReportingGroupSources,
			213 => SplicingNotification,
			0 | 192 | 193 | 255 => Reserved(val),
			_ => Unassigned(val),
		}
	}

	pub fn decode(&self, pkt: &'a [u8]) -> Option<RtcpPacket<'a>> {
		use RtcpType::*;

		match self {
			SenderReport => SenderReportPacket::new(pkt).map(RtcpPacket::SenderReport),
			ReceiverReport => ReceiverReportPacket::new(pkt).map(RtcpPacket::ReceiverReport),
			a => Some(RtcpPacket::KnownType(*a)),
		}
	}

	pub fn decode_mut(&self, pkt: &'a mut [u8]) -> Option<MutableRtcpPacket<'a>> {
		use RtcpType::*;

		match self {
			SenderReport =>
				MutableSenderReportPacket::new(pkt).map(MutableRtcpPacket::SenderReport),
			ReceiverReport =>
				MutableReceiverReportPacket::new(pkt).map(MutableRtcpPacket::ReceiverReport),
			a => Some(MutableRtcpPacket::KnownType(*a)),
		}
	}

	pub fn from_packet(pkt: &[u8]) -> Option<Self> {
		pkt.get(1).cloned().map(Self::new)
	}
}

impl PrimitiveValues for RtcpType {
	type T = (u8,);

	fn to_primitive_values(&self) -> Self::T {
		use RtcpType::*;
		match self {
			SmpteMap => (194,),
			JitterReport => (195,),
			SenderReport => (200,),
			ReceiverReport => (201,),
			SourceDescription => (202,),
			Goodbye => (203,),
			ApplicationDefined => (204,),
			TransportFeedback => (205,),
			PayloadFeedback => (206,),
			ExtendedReport => (207,),
			Avb => (208,),
			ReceiverSummary => (209,),
			PortMapping => (210,),
			Idms => (211,),
			ReportingGroupSources => (212,),
			SplicingNotification => (213,),

			Reserved(val) => (*val,),
			Unassigned(val) => (*val,),
		}
	}
}
