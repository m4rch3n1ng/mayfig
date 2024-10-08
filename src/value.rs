//! contains the [`Value`] enum, a way of deserializing any valid mayfig value

use crate::Error;
use std::{fmt::Debug, hash::Hash};

mod de;
mod map;
mod number;
mod ser;

pub use self::map::Map;
pub use self::number::Number;

/// a representation of a mayfig value
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
	/// represents a string
	String(String),
	/// represents a number
	Number(Number),
	/// represents a boolean
	Bool(bool),
	/// represents a mayfig sequence
	Seq(Vec<Value>),
	/// represents a mayfig map
	Map(Map),
	/// represents a mayfig tagged enum
	///
	/// # warning
	///
	/// serializing this variant via serde will leak the
	/// `tag` string due to a [serde limitation](https://github.com/serde-rs/serde/issues/2218).
	Tagged(String, Vec<Value>),
}

impl Value {
	/// returns a `str` if `self` is [`Value::String`]. returns `None` otherwise.
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Value::String(s) => Some(s),
			_ => None,
		}
	}

	/// returns a [`Number`] if `self` is [`Value::Number`]. returns `None` otherwise.
	pub fn as_number(&self) -> Option<Number> {
		match self {
			Value::Number(num) => Some(*num),
			_ => None,
		}
	}

	/// returns an `f64` if `self` is [`Value::Number`], and casts the value if necessary.
	/// returns `None` otherwise.
	pub fn as_f64(&self) -> Option<f64> {
		match self {
			Value::Number(num) => Some(num.as_f64()),
			_ => None,
		}
	}

	/// returns an `i64` if `self` is [`Value::Number`], the number is an integer and fits into an `i64`.
	/// returns `None` otherwise.
	pub fn as_i64(&self) -> Option<i64> {
		match self {
			Value::Number(num) => num.as_i64(),
			_ => None,
		}
	}

	/// returns a `bool` if `self` is [`Value::Bool`]. returns `None` otherwise.
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Value::Bool(b) => Some(*b),
			_ => None,
		}
	}

	/// returns the associated `&[]` slice if `self` is [`Value::Seq`]. returns `None` otherwise.
	pub fn as_seq(&self) -> Option<&[Value]> {
		match self {
			Value::Seq(seq) => Some(seq),
			_ => None,
		}
	}

	/// returns a mutable reference to the associated `Vec` if `self` is [`Value::Seq`].
	/// returns `None` otherwise.
	pub fn as_seq_mut(&mut self) -> Option<&mut Vec<Value>> {
		match self {
			Value::Seq(seq) => Some(seq),
			_ => None,
		}
	}

	/// returns a [`Map`] if `self` is [`Value::Map`]. returns `None` otherwise.
	pub fn as_map(&self) -> Option<&Map> {
		match self {
			Value::Map(map) => Some(map),
			_ => None,
		}
	}

	/// returns a mutable reference to the associated [`Map`] if `self` is [`Value::Map`].
	/// returns `None` otherwise.
	pub fn as_map_mut(&mut self) -> Option<&mut Map> {
		match self {
			Value::Map(map) => Some(map),
			_ => None,
		}
	}
}

impl Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::String(string) => write!(f, "String({:?})", string),
			Value::Number(number) => Debug::fmt(number, f),
			Value::Bool(bool) => write!(f, "Bool({})", bool),
			Value::Seq(seq) => Debug::fmt(seq, f),
			Value::Map(map) => Debug::fmt(map, f),
			Value::Tagged(tag, values) => write!(f, "Tagged({:?} {:?})", tag, values),
		}
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Value::String(value)
	}
}

impl From<&str> for Value {
	fn from(value: &str) -> Self {
		let owned = value.to_owned();
		Value::String(owned)
	}
}

impl From<u64> for Value {
	fn from(value: u64) -> Self {
		let number = Number::from(value);
		Value::Number(number)
	}
}

impl From<i64> for Value {
	fn from(value: i64) -> Self {
		let number = Number::from(value);
		Value::Number(number)
	}
}

impl From<i32> for Value {
	fn from(value: i32) -> Self {
		Value::from(i64::from(value))
	}
}

impl TryFrom<f64> for Value {
	type Error = Error;
	fn try_from(value: f64) -> Result<Self, Self::Error> {
		let number = Number::try_from(value).map_err(Error::new)?;
		Ok(Value::Number(number))
	}
}

impl From<bool> for Value {
	fn from(value: bool) -> Self {
		Value::Bool(value)
	}
}
