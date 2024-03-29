use super::{read::Read, Deserializer};
use crate::error::Err;
use serde::{
	de::{Deserializer as _, EnumAccess, MapAccess, SeqAccess, VariantAccess},
	forward_to_deserialize_any,
};

pub struct TopMapAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> TopMapAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		TopMapAcc { de }
	}
}

impl<'a, 'de, R: Read<'de>> MapAccess<'de> for TopMapAcc<'a, R> {
	type Error = Err;
	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
	{
		let next = self.de.peek_whitespace()?;
		if next.is_none() {
			return Ok(None);
		}

		let map_key = MapKey::new(self.de);
		seed.deserialize(map_key).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_whitespace()?.ok_or(Err::Eof)?;
		if peek == b'=' {
			self.de.read.discard();
		} else if peek != b'{' && peek != b'[' {
			return Err(Err::Expected('=', char::from(peek)));
		}

		seed.deserialize(&mut *self.de)
	}
}

pub struct MapAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> MapAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		MapAcc { de }
	}
}

impl<'a, 'de, R: Read<'de>> MapAccess<'de> for MapAcc<'a, R> {
	type Error = Err;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
	{
		if self.de.peek_whitespace()?.ok_or(Err::Eof)? == b'}' {
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
		let peek = self.de.peek_whitespace()?.ok_or(Err::Eof)?;
		if peek == b'=' {
			self.de.read.discard();
		} else if peek != b'{' && peek != b'[' {
			return Err(Err::Expected('=', char::from(peek)));
		}

		seed.deserialize(&mut *self.de)
	}
}

pub struct SeqAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> SeqAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		SeqAcc { de }
	}
}

impl<'a, 'de, R: Read<'de>> SeqAccess<'de> for SeqAcc<'a, R> {
	type Error = Err;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		if self.de.peek_whitespace()?.ok_or(Err::Eof)? == b',' {
			self.de.discard_all(b',');
		}

		if self.de.peek_whitespace()?.ok_or(Err::Eof)? == b']' {
			return Ok(None);
		}

		seed.deserialize(&mut *self.de).map(Some)
	}
}

pub struct EnumAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> EnumAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		EnumAcc { de }
	}
}

impl<'a, 'de, R: Read<'de>> EnumAccess<'de> for EnumAcc<'a, R> {
	type Error = Err;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let val = seed.deserialize(&mut *self.de)?;

		let peek = self.de.peek_whitespace()?.ok_or(Err::Eof)?;
		if peek == b'=' {
			self.de.read.discard();
			Ok((val, self))
		} else if let b'{' | b'[' = peek {
			Ok((val, self))
		} else {
			Err(Err::Expected('=', char::from(peek)))
		}
	}
}

#[allow(unused_variables)]
impl<'a, 'de, R: Read<'de>> VariantAccess<'de> for EnumAcc<'a, R> {
	type Error = Err;

	fn unit_variant(self) -> Result<(), Self::Error> {
		<() as serde::de::Deserialize>::deserialize(self.de)
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		seed.deserialize(self.de)
	}

	fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_seq(visitor)
	}

	fn struct_variant<V>(
		self,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_map(visitor)
	}
}

pub struct UnitEnumAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> UnitEnumAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		UnitEnumAcc { de }
	}
}

impl<'a, 'de, R: Read<'de>> EnumAccess<'de> for UnitEnumAcc<'a, R> {
	type Error = Err;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let variant = seed.deserialize(&mut *self.de)?;
		Ok((variant, self))
	}
}

#[allow(unused_variables)]
impl<'a, 'de, R: Read<'de>> VariantAccess<'de> for UnitEnumAcc<'a, R> {
	type Error = Err;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		Err(serde::de::Error::invalid_type(
			serde::de::Unexpected::UnitVariant,
			&"newtype variant",
		))
	}

	fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(serde::de::Error::invalid_type(
			serde::de::Unexpected::UnitVariant,
			&"tuple variant",
		))
	}

	fn struct_variant<V>(
		self,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(serde::de::Error::invalid_type(
			serde::de::Unexpected::UnitVariant,
			&"struct variant",
		))
	}
}

struct MapKey<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> MapKey<'a, R> {
	fn new(de: &'a mut Deserializer<R>) -> Self {
		MapKey { de }
	}
}

#[allow(unused_variables)]
impl<'a, 'de, R: Read<'de>> serde::de::Deserializer<'de> for MapKey<'a, R> {
	type Error = Err;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.de.peek_whitespace()?.ok_or(Err::Eof)?;
		match peek {
			b'[' => self.deserialize_seq(visitor),
			b'{' => self.deserialize_map(visitor),
			b'0'..=b'9' | b'-' | b'.' => self.de.deserialize_number(visitor),
			_ => self.deserialize_str(visitor),
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_bool(visitor)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_i8(visitor)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_i16(visitor)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_i32(visitor)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_i64(visitor)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_u8(visitor)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_u16(visitor)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_u32(visitor)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_u64(visitor)
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_f32(visitor)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_f64(visitor)
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_char(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.de.peek_whitespace()?.ok_or(Err::Eof)?;
		match peek {
			b'"' | b'\'' => self.de.deserialize_str(visitor),
			_ => self.de.deserialize_identifier(visitor),
		}
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_bytes(visitor)
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_byte_buf(visitor)
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_some(self)
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedNone)
	}

	fn deserialize_unit_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedType(name))
	}

	fn deserialize_newtype_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedMapKey("seq"))
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedMapKey("tuple"))
	}

	fn deserialize_tuple_struct<V>(
		self,
		name: &'static str,
		len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedMapKey("tuple"))
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedMapKey("map"))
	}

	fn deserialize_struct<V>(
		self,
		name: &'static str,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedMapKey("struct"))
	}

	fn deserialize_enum<V>(
		self,
		name: &'static str,
		variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Err::UnsupportedMapKey("struct"))
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_identifier(visitor)
	}

	forward_to_deserialize_any! { ignored_any }
}
