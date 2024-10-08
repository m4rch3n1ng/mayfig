use super::{map::MapKey, read::Read};
use crate::{
	error::{Error, ErrorCode},
	Deserializer,
};
use serde::de::{MapAccess, SeqAccess};

pub struct SeqAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> SeqAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		SeqAcc { de }
	}
}

impl<'de, R: Read<'de>> SeqAccess<'de> for SeqAcc<'_, R> {
	type Error = Error;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		self.de.discard_commata();
		if self.de.peek_any().ok_or(Error::EOF)? == b']' {
			return Ok(None);
		}

		seed.deserialize(&mut *self.de).map(Some)
	}
}

pub struct TopMapAcc<'a, R> {
	de: &'a mut Deserializer<R>,
	is_first: bool,
}

impl<'a, 'de, R: Read<'de>> TopMapAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		TopMapAcc { de, is_first: true }
	}
}

impl<'de, R: Read<'de>> MapAccess<'de> for TopMapAcc<'_, R> {
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

		let mut map_key = MapKey::new(self.de);
		seed.deserialize(&mut map_key).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_line()?.ok_or(Error::EOF)?;

		if peek == b'=' {
			self.de.read.discard();
		} else if peek != b'{' {
			let point = self.de.read.position();
			let code = ErrorCode::ExpectedValue(peek as char);
			return Err(Error::with_point(code, point));
		}

		let _ = self.de.peek_line()?.ok_or(Error::EOF)?;
		seed.deserialize(&mut *self.de)
	}
}

pub struct MapAcc<'a, R> {
	de: &'a mut Deserializer<R>,
	is_first: bool,
}

impl<'a, 'de, R: Read<'de>> MapAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		MapAcc { de, is_first: true }
	}
}

impl<'de, R: Read<'de>> MapAccess<'de> for MapAcc<'_, R> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
	{
		if let Ok(Some(b'}')) = self.de.peek_line() {
			self.de.read.discard();
			return Ok(None);
		}

		let peek = if self.is_first {
			self.is_first = false;
			self.de.peek_any()
		} else {
			self.de.peek_newline()?
		};

		if peek.ok_or(Error::EOF)? == b'}' {
			self.de.read.discard();
			return Ok(None);
		}

		let mut map_key = MapKey::new(&mut *self.de);
		seed.deserialize(&mut map_key).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_line()?.ok_or(Error::EOF)?;

		if peek == b'=' {
			self.de.read.discard();
		} else if peek != b'{' {
			let point = self.de.read.position();
			let code = ErrorCode::ExpectedValue(peek as char);
			return Err(Error::with_point(code, point));
		}

		let _ = self.de.peek_line()?.ok_or(Error::EOF)?;
		seed.deserialize(&mut *self.de)
	}
}
