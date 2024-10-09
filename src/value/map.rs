use super::Value;
use indexmap::IndexMap;
use std::{
	fmt::Debug,
	hash::Hash,
	ops::{Deref, DerefMut},
};

#[derive(Clone)]
pub struct Map(pub IndexMap<Value, Value>);

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

impl PartialEq for Map {
	fn eq(&self, other: &Self) -> bool {
		if self.len() != other.len() {
			return false;
		}

		for (l, r) in self.iter().zip(other.iter()) {
			if l != r {
				return false;
			}
		}

		true
	}
}

impl Eq for Map {}

impl Hash for Map {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_usize(self.0.len());
		for elt in &self.0 {
			elt.hash(state)
		}
	}
}
