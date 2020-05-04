use pnet_macros_support::packet::PrimitiveValues;

include!(concat!(env!("OUT_DIR"), "/rtcp.rs"));

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// RTCP message types. These define the packet format used for the payload.
pub enum RtcpType {
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

	/// RTPFB, feedback on the transport layer.
	///
	/// See [RFC 4845](https://tools.ietf.org/html/rfc4585)
	///
	/// Code 205.
	TransportFeedback,

	/// PSFB, feedback on the payload.
	///
	/// See [RFC 4845](https://tools.ietf.org/html/rfc4585)
	///
	/// Code 206.
	PayloadFeedback,

	/// Extended Report message, used for additional/mixed report blocks.
	///
	/// See [RTCP XR](https://tools.ietf.org/html/rfc3611).
	///
	/// Code 207.
	ExtendedReport,

	/// Receiver Summary information.
	///
	/// See [RFC 5760](https://tools.ietf.org/html/rfc5760).
	///
	/// Code 209.
	ReceiverSummary,

	/// Unknown message type.
	Other(u8),
}

impl RtcpType {
	pub fn new(val: u8) -> Self {
		match val {
			200 => Self::SenderReport,
			201 => Self::ReceiverReport,
			202 => Self::SourceDescription,
			203 => Self::Goodbye,
			204 => Self::ApplicationDefined,
			205 => Self::TransportFeedback,
			206 => Self::PayloadFeedback,
			207 => Self::ExtendedReport,
			209 => Self::ReceiverSummary,
			_ => Self::Other(val),
		}
	}
}

impl PrimitiveValues for RtcpType {
	type T = (u8,);

	fn to_primitive_values(&self) -> Self::T {
		match self {
			Self::SenderReport => (200,),
			Self::ReceiverReport => (201,),
			Self::SourceDescription => (202,),
			Self::Goodbye => (203,),
			Self::ApplicationDefined => (204,),
			Self::TransportFeedback => (205,),
			Self::PayloadFeedback => (206,),
			Self::ExtendedReport => (207,),
			Self::ReceiverSummary => (209,),
			Self::Other(val) => (*val,),
		}
	}
}