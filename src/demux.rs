//! Utilities for separating multiplexed RTP/RTCP streams.
//!
//! *These are included when using the `"demux"` feature.*

use crate::{
	rtcp::{MutableRtcpPacket, RtcpPacket, RtcpType},
	rtp::{MutableRtpPacket, RtpPacket, RtpType},
};

/// RTP/RTCP packets separated from the same stream.
///
/// `Failed` variants arise if too few bytes were provided to decode the first header.
#[derive(Debug)]
pub enum Demuxed<'a> {
	Rtp(RtpPacket<'a>),
	Rtcp(RtcpPacket<'a>),
	FailedParse(DemuxType),
	TooSmall,
}

/// Mutable RTP/RTCP packets separated from the same stream.
///
/// `Failed` variants arise if too few bytes were provided to decode the first header.
#[derive(Debug)]
pub enum DemuxedMut<'a> {
	Rtp(MutableRtpPacket<'a>),
	Rtcp(MutableRtcpPacket<'a>),
	FailedParse(DemuxType),
	TooSmall,
}

/// Demultiplexes combined RTP and RTCP streams.
///
/// This is subject to the profile restrictions under [RFC 5761],
/// which restricts the set of allowed payload types. In particular,
/// this implementation returns an [`RtcpPacket`]
/// if its packet type matches any known [RTCP packet type].
///
/// Returns `None` if the `pkt` is too short (less than 2 bytes).
///
/// [RFC 5761]: https://tools.ietf.org/html/rfc5761#section-4
/// [`RtcpPacket`]: ../rtcp/struct.RtcpPacket.html
/// [RTCP packet type]: ../rtcp/enum.RtcpType.html
#[must_use]
pub fn demux(pkt: &[u8]) -> Demuxed {
	if pkt.len() < 2 {
		Demuxed::TooSmall
	} else {
		let pt = classify_pt(pkt);
		match pt {
			DemuxType::Rtp(_) => RtpPacket::new(pkt).map(Demuxed::Rtp),
			DemuxType::Rtcp(rt) => rt.decode(pkt).map(Demuxed::Rtcp),
		}
		.unwrap_or(Demuxed::FailedParse(pt))
	}
}

/// Demultiplexes combined RTP and RTCP streams, returning mutable packets.
///
/// See [`demux`] for more information.
///
/// [`demux`]: fn.demux.html
pub fn demux_mut(pkt: &mut [u8]) -> DemuxedMut {
	if pkt.len() < 2 {
		DemuxedMut::TooSmall
	} else {
		let pt = classify_pt(pkt);
		match pt {
			DemuxType::Rtp(_) => MutableRtpPacket::new(pkt).map(DemuxedMut::Rtp),
			DemuxType::Rtcp(rt) => rt.decode_mut(pkt).map(DemuxedMut::Rtcp),
		}
		.unwrap_or(DemuxedMut::FailedParse(pt))
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DemuxType {
	Rtp(RtpType),
	Rtcp(RtcpType),
}

// Returns true if RTP.
#[inline]
fn classify_pt(pkt: &[u8]) -> DemuxType {
	match RtcpType::new(pkt[1]) {
		RtcpType::Reserved(a) | RtcpType::Unassigned(a) =>
			DemuxType::Rtp(RtpType::new(a & 0b0111_1111)),
		a => DemuxType::Rtcp(a),
	}
}
