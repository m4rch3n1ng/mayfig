use super::read::Read;
use crate::{error::Error, Deserializer};
use serde::de::MapAccess;

pub struct TopMapAcc<'a, R> {
	de: &'a mut Deserializer<R>,
	is_first: bool,
}

impl<'a, 'de, R: Read<'de>> TopMapAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		TopMapAcc { de, is_first: true }
	}
}

impl<'a, 'de, R: Read<'de>> MapAccess<'de> for TopMapAcc<'a, R> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
	{
		let peek = if self.is_first {
			self.is_first = false;
			self.de.peek_any()
		} else {
			self.de.peek_newline()?
		};
		if peek.is_none() {
			return Ok(None);
		}

		seed.deserialize(&mut *self.de).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_line()?.ok_or(Error::Eof)?;

		if peek == b'=' {
			self.de.read.discard();
		} else if peek != b'{' {
			return Err(Error::ExpectedValue(peek as char));
		}

		seed.deserialize(&mut *self.de)
	}
}
