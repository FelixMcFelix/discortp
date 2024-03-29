use crate::rtcp::RtcpType;
use alloc::vec::Vec;
use pnet_macros::packet;
use pnet_macros_support::types::{u1, u16be, u2, u24be, u32be, u5};

#[packet]
#[derive(Eq, PartialEq)]
/// Sender report, containing jitter, reception, timing and volume information.
///
/// See the relevant [RTP RFC section](https://tools.ietf.org/html/rfc3550#section-6.4.1).
/// Main payload body is a [`SenderInfo`] block followed by [`rx_report_count`] x [`ReportBlock`].
///
/// A description of fields:
///
/// ## `version`
/// RTP version. Should be `2`.
///
/// ## `padding`
/// Packet contains padding octets which are not part of the payload, but
/// who are counted in [`length`]. The last byte of the payload contains the
/// count of bytes to be ignored from the end (including itself).
///
/// ## `rx_report_count`
/// Number of [`ReportBlock`]s contained. May be `0`.
///
/// ## `packet_type`
/// Must be [`RtcpType::SenderReport`].
///
/// ## `pkt_length`
/// Length of this RTCP packet in 32-bit words, minus one.
/// Includes header and padding. `0` is valid for compound RTCP packets.
///
/// ## `ssrc`
/// SSRC for the source of this packet.
///
/// ## `payload`
/// Bytes of the RTCP body.
///
/// [`SenderInfo`]: struct.SenderInfo.html
/// [`rx_report_count`]: #structfield.rx_report_count
/// [`ReportBlock`]: struct.ReportBlock.html
/// [`length`]: #structfield.length
/// [`RtcpType::SenderReport`]: ../enum.RtcpType.html#variant.SenderReport
pub struct SenderReport {
	pub version: u2,

	pub padding: u1,

	pub rx_report_count: u5,

	#[construct_with(u8)]
	pub packet_type: RtcpType,

	pub pkt_length: u16be,

	pub ssrc: u32be,

	#[payload]
	pub payload: Vec<u8>,
}

#[packet]
#[derive(Eq, PartialEq)]
/// Receiver report, containing jitter and reception information.
///
/// See the relevant [RTP RFC section](https://tools.ietf.org/html/rfc3550#section-6.4.2).
/// Main payload body is [`rx_report_count`] x [`ReportBlock`].
///
/// A description of fields:
///
/// ## `version`
/// RTP version. Should be `2`.
///
/// ## `padding`
/// Packet contains padding octets which are not part of the payload, but
/// who are counted in [`length`]. The last byte of the payload contains the
/// count of bytes to be ignored from the end (including itself).
///
/// ## `rx_report_count`
/// Number of [`ReportBlock`]s contained. May be `0`.
///
/// ## `packet_type`
/// Must be [`RtcpType::SenderReport`].
///
/// ## `pkt_length`
/// Length of this RTCP packet in 32-bit words, minus one.
/// Includes header and padding. `0` is valid for compound RTCP packets.
///
/// ## `ssrc`
/// SSRC for the source of this packet.
///
/// ## `payload`
/// Bytes of the RTCP body.
///
/// [`SenderInfo`]: struct.SenderInfo.html
/// [`rx_report_count`]: #structfield.rx_report_count
/// [`ReportBlock`]: struct.ReportBlock.html
/// [`length`]: #structfield.length
/// [`RtcpType::SenderReport`]: ../enum.RtcpType.html#variant.SenderReport
pub struct ReceiverReport {
	pub version: u2,

	pub padding: u1,

	pub rx_report_count: u5,

	#[construct_with(u8)]
	pub packet_type: RtcpType,

	pub pkt_length: u16be,

	pub ssrc: u32be,

	#[payload]
	pub payload: Vec<u8>,
}

#[packet]
#[derive(Eq, PartialEq)]
/// Sender Info block in a [`SenderReport`].
///
/// A description of fields:
///
/// ## `ntp_timestamp_{second,fraction}`
/// Wallclock time when this report was sent.
///
/// ## `rtp_timestamp`
/// The above timestamp, converted to use the same units/offset
/// as the timestamps appearing in RTP packets.
///
/// ## `pkt_count`
/// Total number of packets sent by this sender since the start of the session.
///
/// Should be reset if SSRC changes.
///
/// ## `byte_count`
/// Total number of *payload* bytes/octets sent by this sender.
///
/// Should be reset if SSRC changes.
///
/// ## `payload`
/// Remainder of the packet.
///
/// [`SenderReport`]: struct.SenderReport.html
/// [`rx_report_count`]: #structfield.rx_report_count
/// [`ReportBlock`]: struct.ReportBlock.html
/// [`length`]: #structfield.length
/// [`RtcpType::SenderReport`]: ../enum.RtcpType.html#variant.SenderReport
pub struct SenderInfo {
	pub ntp_timestamp_second: u32be,

	pub ntp_timestamp_fraction: u32be,

	pub rtp_timestamp: u32be,

	pub pkt_count: u32be,

	pub byte_count: u32be,

	#[payload]
	pub payload: Vec<u8>,
}

#[packet]
#[derive(Eq, PartialEq)]
/// Report block in a [`SenderReport`] or [`ReceiverReport`].
///
/// A description of fields:
///
/// ## `ssrc`
/// SSRC of stream which this report concerns.
///
/// ## `fraction_lost`
/// Packet loss as a fixed-point number (*i.e.*, n => n/256).
///
/// ## `cumulative_pkts_lost`
/// Total number of packets from this SSRC that have been lost since
/// reception began.
///
/// ## `cycles`
/// Number of times the source's sequence number has wrapped past its
/// starting value.
///
/// Part of the "extended highest sequence number received".
///
/// ## `sequence`
/// Highest sequence number observed from this source.
///
/// Part of the "extended highest sequence number received".
///
/// ## `interarrival_jitter`
/// Estimated variance of RTP interarrival times.
///
/// See p.40 of [the RFC] for the algorithm used to calculate this.
///
/// ## `last_sr_timestamp`
/// Middle 32 bits of the NTP timestamp of the most recent sender report
/// from this SSRC.
///
/// Will be `0` if no report has been observed.
///
/// ## `last_sr_delay`
/// Delay in fractions of a second (*i.e.*,`n/65536`) since the last sender report
/// from this SSRC was received.
///
/// Set to `0` *iff.* no send report is observed
///
/// ## `payload`
/// Remainder of the packet.
///
/// [`SenderReport`]: struct.SenderReport.html
/// [`ReceiverReport`]: struct.ReceiverReport.html
/// [`rx_report_count`]: #structfield.rx_report_count
/// [`ReportBlock`]: struct.ReportBlock.html
/// [`length`]: #structfield.length
/// [`RtcpType::SenderReport`]: ../enum.RtcpType.html#variant.SenderReport
/// [the RFC]: https://tools.ietf.org/html/rfc3550
pub struct ReportBlock {
	pub ssrc: u32be,

	pub fraction_lost: u8,

	pub cumulative_pkts_lost: u24be,

	pub cycles: u16be,

	pub sequence: u16be,

	pub interarrival_jitter: u32be,

	pub last_sr_timestamp: u32be,

	pub last_sr_delay: u32be,

	#[payload]
	pub payload: Vec<u8>,
}
