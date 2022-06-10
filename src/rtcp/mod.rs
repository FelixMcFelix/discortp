//! Readers and writers for the [RTP Control Protocol](https://tools.ietf.org/html/rfc3550#section-6).
//!
//! *These are included when using the `"rtcp"` feature.*

pub mod report;
use crate::{FromPacket, MutablePacket, Packet, PacketSize};
use pnet_macros_support::packet::PrimitiveValues;
use report::{
	MutableReceiverReportPacket,
	MutableSenderReportPacket,
	ReceiverReport,
	ReceiverReportPacket,
	SenderReport,
	SenderReportPacket,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Rtcp {
	SenderReport(SenderReport),
	ReceiverReport(ReceiverReport),

	KnownType(RtcpType),
}

/// RTCP packet variants separated from the same stream.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum RtcpPacket<'a> {
	SenderReport(SenderReportPacket<'a>),
	ReceiverReport(ReceiverReportPacket<'a>),

	KnownType(RtcpType),
}

impl RtcpPacket<'_> {
	#[must_use]
	pub fn new(pkt: &[u8]) -> Option<RtcpPacket<'_>> {
		RtcpType::from_packet(pkt).and_then(|rtcp_id| rtcp_id.decode(pkt))
	}
}

impl<'a> Packet for RtcpPacket<'a> {
	fn packet(&self) -> &[u8] {
		match self {
			Self::SenderReport(s) => s.packet(),
			Self::ReceiverReport(s) => s.packet(),
			Self::KnownType(_) => &[],
		}
	}

	fn payload(&self) -> &[u8] {
		match self {
			Self::SenderReport(s) => s.payload(),
			Self::ReceiverReport(s) => s.payload(),
			Self::KnownType(_) => &[],
		}
	}
}

impl<'a> FromPacket for RtcpPacket<'a> {
	type T = Rtcp;

	fn from_packet(&self) -> Self::T {
		match self {
			Self::SenderReport(s) => Rtcp::SenderReport(s.from_packet()),
			Self::ReceiverReport(s) => Rtcp::ReceiverReport(s.from_packet()),
			Self::KnownType(t) => Rtcp::KnownType(*t),
		}
	}
}

impl<'a> PacketSize for RtcpPacket<'a> {
	fn packet_size(&self) -> usize {
		match self {
			Self::SenderReport(s) => s.packet_size(),
			Self::ReceiverReport(s) => s.packet_size(),
			Self::KnownType(_) => 0,
		}
	}
}

/// Mutable RTP/RTCP packets separated from the same stream.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum MutableRtcpPacket<'a> {
	SenderReport(MutableSenderReportPacket<'a>),
	ReceiverReport(MutableReceiverReportPacket<'a>),

	KnownType(RtcpType),
}

impl MutableRtcpPacket<'_> {
	pub fn new(pkt: &mut [u8]) -> Option<MutableRtcpPacket<'_>> {
		RtcpType::from_packet(pkt).and_then(move |rtcp_id| rtcp_id.decode_mut(pkt))
	}
}

impl<'a> Packet for MutableRtcpPacket<'a> {
	fn packet(&self) -> &[u8] {
		match self {
			Self::SenderReport(s) => s.packet(),
			Self::ReceiverReport(s) => s.packet(),
			Self::KnownType(_) => &[],
		}
	}

	fn payload(&self) -> &[u8] {
		match self {
			Self::SenderReport(s) => s.payload(),
			Self::ReceiverReport(s) => s.payload(),
			Self::KnownType(_) => &[],
		}
	}
}

impl<'a> MutablePacket for MutableRtcpPacket<'a> {
	fn packet_mut(&mut self) -> &mut [u8] {
		match self {
			Self::SenderReport(s) => s.packet_mut(),
			Self::ReceiverReport(s) => s.packet_mut(),
			Self::KnownType(_) => &mut [],
		}
	}

	fn payload_mut(&mut self) -> &mut [u8] {
		match self {
			Self::SenderReport(s) => s.payload_mut(),
			Self::ReceiverReport(s) => s.payload_mut(),
			Self::KnownType(_) => &mut [],
		}
	}
}

impl<'a> FromPacket for MutableRtcpPacket<'a> {
	type T = Rtcp;

	fn from_packet(&self) -> Self::T {
		match self {
			Self::SenderReport(s) => Rtcp::SenderReport(s.from_packet()),
			Self::ReceiverReport(s) => Rtcp::ReceiverReport(s.from_packet()),
			Self::KnownType(t) => Rtcp::KnownType(*t),
		}
	}
}

impl<'a> PacketSize for MutableRtcpPacket<'a> {
	fn packet_size(&self) -> usize {
		match self {
			Self::SenderReport(s) => s.packet_size(),
			Self::ReceiverReport(s) => s.packet_size(),
			Self::KnownType(_) => 0,
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
	#[must_use]
	pub fn new(val: u8) -> Self {
		match val {
			194 => Self::SmpteMap,
			195 => Self::JitterReport,
			200 => Self::SenderReport,
			201 => Self::ReceiverReport,
			202 => Self::SourceDescription,
			203 => Self::Goodbye,
			204 => Self::ApplicationDefined,
			205 => Self::TransportFeedback,
			206 => Self::PayloadFeedback,
			207 => Self::ExtendedReport,
			208 => Self::Avb,
			209 => Self::ReceiverSummary,
			210 => Self::PortMapping,
			211 => Self::Idms,
			212 => Self::ReportingGroupSources,
			213 => Self::SplicingNotification,
			0 | 192 | 193 | 255 => Self::Reserved(val),
			_ => Self::Unassigned(val),
		}
	}

	#[must_use]
	pub fn decode(&self, pkt: &'a [u8]) -> Option<RtcpPacket<'a>> {
		match self {
			Self::SenderReport => SenderReportPacket::new(pkt).map(RtcpPacket::SenderReport),
			Self::ReceiverReport => ReceiverReportPacket::new(pkt).map(RtcpPacket::ReceiverReport),
			a => Some(RtcpPacket::KnownType(*a)),
		}
	}

	pub fn decode_mut(&self, pkt: &'a mut [u8]) -> Option<MutableRtcpPacket<'a>> {
		match self {
			Self::SenderReport =>
				MutableSenderReportPacket::new(pkt).map(MutableRtcpPacket::SenderReport),
			Self::ReceiverReport =>
				MutableReceiverReportPacket::new(pkt).map(MutableRtcpPacket::ReceiverReport),
			a => Some(MutableRtcpPacket::KnownType(*a)),
		}
	}

	pub fn from_packet(pkt: &[u8]) -> Option<Self> {
		pkt.get(1).copied().map(Self::new)
	}
}

impl PrimitiveValues for RtcpType {
	type T = (u8,);

	fn to_primitive_values(&self) -> Self::T {
		match self {
			Self::SmpteMap => (194,),
			Self::JitterReport => (195,),
			Self::SenderReport => (200,),
			Self::ReceiverReport => (201,),
			Self::SourceDescription => (202,),
			Self::Goodbye => (203,),
			Self::ApplicationDefined => (204,),
			Self::TransportFeedback => (205,),
			Self::PayloadFeedback => (206,),
			Self::ExtendedReport => (207,),
			Self::Avb => (208,),
			Self::ReceiverSummary => (209,),
			Self::PortMapping => (210,),
			Self::Idms => (211,),
			Self::ReportingGroupSources => (212,),
			Self::SplicingNotification => (213,),

			Self::Reserved(val) | Self::Unassigned(val) => (*val,),
		}
	}
}
