use super::read::Read;
use crate::{error::Error, Deserializer};
use serde::de::MapAccess;

pub struct TopMapAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> TopMapAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		TopMapAcc { de }
	}
}

impl<'a, 'de, R: Read<'de>> MapAccess<'de> for TopMapAcc<'a, R> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
	{
		let Some(_) = self.de.peek_any() else {
			return Ok(None);
		};

		seed.deserialize(&mut *self.de).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_line()?;

		if peek == b'=' {
			self.de.read.discard();
		} else if peek != b'{' {
			return Err(Error::ExpectedValue(peek as char));
		}

		seed.deserialize(&mut *self.de)
	}
}
