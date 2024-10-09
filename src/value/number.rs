use serde::Serialize;

use crate::error::ErrorCode;
use std::{
	fmt::{Debug, Display},
	hash::Hash,
};

/// represents a mayfig number. can be positive, negative or a float
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Number(InternalNumber);

impl Number {
	/// returns an `f64`, casting the value if necessary.
	pub fn as_f64(&self) -> f64 {
		match self.0 {
			InternalNumber::PosInt(u) => u as f64,
			InternalNumber::NegInt(i) => i as f64,
			InternalNumber::Float(ff) => ff,
		}
	}

	/// returns an `i64` if the number is an integer and fits into an `i64`.
	pub fn as_i64(&self) -> Option<i64> {
		match self.0 {
			InternalNumber::PosInt(u) => {
				if u <= i64::MAX as u64 {
					Some(u as i64)
				} else {
					None
				}
			}
			InternalNumber::NegInt(i) => Some(i),
			InternalNumber::Float(_) => None,
		}
	}

	/// returns a `u64` if the number is a positive integer.
	pub fn as_u64(&self) -> Option<u64> {
		match self.0 {
			InternalNumber::PosInt(u) => Some(u),
			InternalNumber::NegInt(_) | InternalNumber::Float(_) => None,
		}
	}
}

impl Serialize for Number {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self.0 {
			InternalNumber::PosInt(u) => serializer.serialize_u64(u),
			InternalNumber::NegInt(i) => serializer.serialize_i64(i),
			InternalNumber::Float(ff) => serializer.serialize_f64(ff),
		}
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Number({})", self.0)
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InternalNumber {
	PosInt(u64),
	NegInt(i64),
	Float(f64),
}

impl Display for InternalNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			InternalNumber::PosInt(u) => Display::fmt(u, f),
			InternalNumber::NegInt(i) => Display::fmt(i, f),
			InternalNumber::Float(ff) => Display::fmt(ff, f),
		}
	}
}

// InternalNumber::Float cannot be NaN
impl Eq for InternalNumber {}

impl Hash for InternalNumber {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		std::mem::discriminant(self).hash(state);
		match self {
			InternalNumber::PosInt(u) => u.hash(state),
			InternalNumber::NegInt(i) => i.hash(state),
			InternalNumber::Float(ff) => {
				let bits = if *ff == 0. {
					// to hold the +0.0 == -0.0 => hash(+0.0) == hash(-0.0) contract
					// https://doc.rust-lang.org/beta/std/hash/trait.Hash.html#hash-and-eq
					0
				} else {
					// InternalNumber::Float cannot be NaN
					ff.to_bits()
				};
				bits.hash(state)
			}
		}
	}
}

impl From<u64> for Number {
	fn from(value: u64) -> Self {
		let internal = InternalNumber::PosInt(value);
		Number(internal)
	}
}

impl From<i64> for Number {
	fn from(val: i64) -> Self {
		let internal = if let Ok(pos) = u64::try_from(val) {
			InternalNumber::PosInt(pos)
		} else {
			InternalNumber::NegInt(val)
		};

		Number(internal)
	}
}

impl From<i32> for Number {
	fn from(value: i32) -> Self {
		Number::from(i64::from(value))
	}
}

impl TryFrom<f64> for Number {
	type Error = ErrorCode;
	fn try_from(value: f64) -> Result<Self, Self::Error> {
		if value.is_nan() {
			return Err(ErrorCode::UnsupportedNaN);
		}

		let internal = InternalNumber::Float(value);
		Ok(Number(internal))
	}
}
