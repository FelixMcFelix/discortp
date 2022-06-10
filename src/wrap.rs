//! Utility types for wrapping arithmetic, compatible with pnet.

use core::{
	num::Wrapping,
	ops::{Add, AddAssign, Sub, SubAssign},
};
use pnet_macros_support::{
	packet::PrimitiveValues,
	types::{u16be, u32be},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wrap16(pub Wrapping<u16>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wrap32(pub Wrapping<u32>);

impl Wrap16 {
	#[must_use]
	pub fn new(v: u16be) -> Self {
		Self(Wrapping(v))
	}
}

impl From<Wrap16> for u16 {
	fn from(val: Wrap16) -> Self {
		(val.0).0
	}
}

impl From<u16> for Wrap16 {
	fn from(val: u16) -> Self {
		Wrap16(Wrapping(val))
	}
}

impl PrimitiveValues for Wrap16 {
	type T = (u16be,);

	fn to_primitive_values(&self) -> Self::T {
		((*self).into(),)
	}
}

impl Add<u16> for Wrap16 {
	type Output = Self;

	fn add(self, other: u16) -> Self::Output {
		Wrap16(self.0 + Wrapping(other))
	}
}

impl AddAssign<u16> for Wrap16 {
	fn add_assign(&mut self, other: u16) {
		self.0 += Wrapping(other);
	}
}

impl Sub<u16> for Wrap16 {
	type Output = Self;

	fn sub(self, other: u16) -> Self::Output {
		Wrap16(self.0 - Wrapping(other))
	}
}

impl SubAssign<u16> for Wrap16 {
	fn sub_assign(&mut self, other: u16) {
		self.0 -= Wrapping(other);
	}
}

impl Wrap32 {
	#[must_use]
	pub fn new(v: u32be) -> Self {
		Self(Wrapping(v))
	}
}

impl From<Wrap32> for u32 {
	fn from(val: Wrap32) -> Self {
		(val.0).0
	}
}

impl From<u32> for Wrap32 {
	fn from(val: u32) -> Self {
		Wrap32(Wrapping(val))
	}
}

impl PrimitiveValues for Wrap32 {
	type T = (u32be,);

	fn to_primitive_values(&self) -> Self::T {
		((*self).into(),)
	}
}

impl Add<u32> for Wrap32 {
	type Output = Self;

	fn add(self, other: u32) -> Self::Output {
		Wrap32(self.0 + Wrapping(other))
	}
}

impl AddAssign<u32> for Wrap32 {
	fn add_assign(&mut self, other: u32) {
		self.0 += Wrapping(other);
	}
}

impl Sub<u32> for Wrap32 {
	type Output = Self;

	fn sub(self, other: u32) -> Self::Output {
		Wrap32(self.0 - Wrapping(other))
	}
}

impl SubAssign<u32> for Wrap32 {
	fn sub_assign(&mut self, other: u32) {
		self.0 -= Wrapping(other);
	}
}
