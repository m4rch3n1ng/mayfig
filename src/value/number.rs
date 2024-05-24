use crate::error::ErrorCode;
use std::{
	fmt::{Debug, Display},
	hash::Hash,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number(InternalNumber);

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

#[derive(Debug, Clone, PartialEq)]
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
	#[expect(unused_variables)]
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		todo!()
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
