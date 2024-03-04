use super::Deserializer;
use crate::error::Err;
use serde::{
	de::{EnumAccess, MapAccess, SeqAccess, VariantAccess},
	forward_to_deserialize_any,
};

pub struct TopMapAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> TopMapAcc<'a, 'de> {
	pub fn new(de: &'a mut Deserializer<'de>) -> Self {
		TopMapAcc { de }
	}
}

impl<'a, 'de> MapAccess<'de> for TopMapAcc<'a, 'de> {
	type Error = Err;
	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
	{
		let next = self.de.peek_whitespace();
		let Ok(next) = next else {
			return Ok(None);
		};

		if next == ';' {
			self.de.discard_all(';');
		}

		let next = self.de.peek_whitespace();
		if next.is_err() {
			return Ok(None);
		}

		seed.deserialize(&mut *self.de).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_whitespace()?;
		if peek == '=' {
			self.de.read.discard();
		} else if peek != '{' && peek != '[' {
			return Err(Err::Expected('=', peek));
		}

		seed.deserialize(&mut *self.de)
	}
}

pub struct MapAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAcc<'a, 'de> {
	pub fn new(de: &'a mut Deserializer<'de>) -> Self {
		MapAcc { de }
	}
}

impl<'a, 'de> MapAccess<'de> for MapAcc<'a, 'de> {
	type Error = Err;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
	{
		if self.de.peek_whitespace()? == ';' {
			self.de.discard_all(';');
		}

		if self.de.peek_whitespace()? == '}' {
			self.de.read.discard();
			return Ok(None);
		}

		let map_key = MapKey::new(self.de);
		seed.deserialize(map_key).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_whitespace()?;
		if peek == '=' {
			self.de.read.discard();
		} else if peek != '{' && peek != '[' {
			return Err(Err::Expected('=', peek));
		}

		seed.deserialize(&mut *self.de)
	}
}

pub struct SeqAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SeqAcc<'a, 'de> {
	pub fn new(de: &'a mut Deserializer<'de>) -> Self {
		SeqAcc { de }
	}
}

impl<'a, 'de> SeqAccess<'de> for SeqAcc<'a, 'de> {
	type Error = Err;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		if self.de.peek_whitespace()? == ',' {
			self.de.discard_all(',');
		}

		if self.de.peek_whitespace()? == ']' {
			self.de.read.discard();
			return Ok(None);
		}

		seed.deserialize(&mut *self.de).map(Some)
	}
}

pub struct EnumAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> EnumAcc<'a, 'de> {
	pub fn new(de: &'a mut Deserializer<'de>) -> Self {
		EnumAcc { de }
	}
}

impl<'a, 'de> EnumAccess<'de> for EnumAcc<'a, 'de> {
	type Error = Err;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let val = seed.deserialize(&mut *self.de)?;

		let peek = self.de.peek_whitespace()?;
		if peek == '=' {
			self.de.read.discard();
			Ok((val, self))
		} else if let '{' | '[' = peek {
			Ok((val, self))
		} else {
			Err(Err::Expected('=', peek))
		}
	}
}

#[allow(unused_variables)]
impl<'a, 'de> VariantAccess<'de> for EnumAcc<'a, 'de> {
	type Error = Err;

	fn unit_variant(self) -> Result<(), Self::Error> {
		todo!()
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		todo!()
	}

	fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn struct_variant<V>(
		self,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		serde::de::Deserializer::deserialize_map(self.de, visitor)
	}
}

struct MapKey<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapKey<'a, 'de> {
	fn new(de: &'a mut Deserializer<'de>) -> Self {
		MapKey { de }
	}
}

impl<'a, 'de> serde::de::Deserializer<'de> for MapKey<'a, 'de> {
	type Error = Err;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_any(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.de.peek_whitespace()?;

		match peek {
			'"' => self.de.deserialize_str(visitor),
			_ => self.de.deserialize_identifier(visitor),
		}
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	forward_to_deserialize_any! {
		bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
		char bytes byte_buf
		option unit unit_struct newtype_struct
		seq tuple tuple_struct map struct enum
		identifier ignored_any
	}
}
