use crate::Error;
use indexmap::IndexMap;
use std::{
	fmt::Debug,
	hash::Hash,
	ops::{Deref, DerefMut},
};

mod number;

pub use self::number::Number;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
	String(String),
	Number(Number),
	Bool(bool),
	Seq(Seq),
	Map(Map),
}

impl Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::String(string) => write!(f, "String({:?})", string),
			Value::Number(number) => Debug::fmt(number, f),
			Value::Bool(bool) => write!(f, "Bool({})", bool),
			Value::Seq(seq) => Debug::fmt(seq, f),
			Value::Map(map) => Debug::fmt(map, f),
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Seq(Vec<Value>);

impl Seq {
	pub const fn new() -> Self {
		Seq(Vec::new())
	}
}

impl Debug for Seq {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Seq ")?;
		Debug::fmt(&self.0, f)
	}
}

impl Deref for Seq {
	type Target = Vec<Value>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Seq {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Default for Seq {
	fn default() -> Self {
		Seq::new()
	}
}

impl From<Vec<Value>> for Seq {
	fn from(value: Vec<Value>) -> Self {
		Seq(value)
	}
}

impl<const N: usize> From<[Value; N]> for Seq {
	fn from(value: [Value; N]) -> Self {
		Seq(value.to_vec())
	}
}

impl From<&[Value]> for Seq {
	fn from(value: &[Value]) -> Self {
		Seq(value.to_owned())
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Map(IndexMap<Value, Value>);

impl Map {
	pub fn new() -> Self {
		Map(IndexMap::new())
	}
}

impl Debug for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Map ")?;
		Debug::fmt(&self.0, f)
	}
}

impl Deref for Map {
	type Target = IndexMap<Value, Value>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Map {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Default for Map {
	fn default() -> Self {
		Map::new()
	}
}

impl<const N: usize> From<[(Value, Value); N]> for Map {
	fn from(value: [(Value, Value); N]) -> Self {
		let map = IndexMap::from(value);
		Map(map)
	}
}

impl Hash for Map {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_usize(self.0.len());
		for elt in &self.0 {
			elt.hash(state)
		}
	}
}
