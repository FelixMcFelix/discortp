//! Utilities for separating multiplexed RTP/RTCP streams.
//!
//! *These are included when using the `"demux"` feature.*

use crate::{
	rtcp::{
		MutableRtcpPacket,
		RtcpPacket,
		RtcpType,
	},
	rtp::{
		MutableRtpPacket,
		RtpPacket,
	},
};

/// RTP/RTCP packets separated from the same stream.
pub enum Demuxed<'a> {
	Rtp(RtpPacket<'a>),
	Rtcp(RtcpPacket<'a>),
}

/// Mutable RTP/RTCP packets separated from the same stream.
pub enum DemuxedMut<'a> {
	Rtp(MutableRtpPacket<'a>),
	Rtcp(MutableRtcpPacket<'a>),
}

/// Demultiplexes combined RTP and RTCP streams.
///
/// This is subject to the profile restrictions under [RFC 5761],
/// which restricts the set of allowed payload types. In particular,
/// this implementation returns an [`RtcpPacket`]
/// if its packet type matches any known [RTCP packet type].
///
/// [RFC 5761]: https://tools.ietf.org/html/rfc5761#section-4
/// [`RtcpPacket`]: rtcp/struct.RtcpPacket.html
/// [RTCP packet type]: rtcp/enum.RtcpType.html
pub fn demux(pkt: &[u8]) -> Option<Demuxed> {
	if pkt.len() < 2 {
		None
	} else {
		if classify_pt(pkt) {
			RtpPacket::new(pkt)
				.map(Demuxed::Rtp)
		} else {
			RtcpPacket::new(pkt)
				.map(Demuxed::Rtcp)
		}
	}
}

/// Demultiplexes combined RTP and RTCP streams, returning mutable packets.
///
/// See [`demux`] for more information.
///
/// [`demux`]: fn.demux.html
pub fn demux_mut(pkt: &mut [u8]) -> Option<DemuxedMut> {
	if pkt.len() < 2 {
		None
	} else {
		if classify_pt(pkt) {
			MutableRtpPacket::new(pkt)
				.map(DemuxedMut::Rtp)
		} else {
			MutableRtcpPacket::new(pkt)
				.map(DemuxedMut::Rtcp)
		}
	}
}

// Returns true if RTP.
#[inline]
fn classify_pt(pkt: &[u8]) -> bool {
	matches!(RtcpType::new(pkt[1]), RtcpType::Reserved(_) | RtcpType::Unassigned(_))
}