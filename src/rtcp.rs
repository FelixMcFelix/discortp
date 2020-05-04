use pnet_macros_support::packet::PrimitiveValues;

include!(concat!(env!("OUT_DIR"), "/rtcp.rs"));

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl RtcpType {
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