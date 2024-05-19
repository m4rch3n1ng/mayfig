use super::{access::SeqAcc, read::Read};
use crate::{error::Error, Deserializer};
use serde::{
	de::{Deserializer as _, EnumAccess, VariantAccess},
	forward_to_deserialize_any,
};

pub struct TaggedEnumAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> TaggedEnumAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		TaggedEnumAcc { de }
	}
}

impl<'a, 'de, R: Read<'de>> EnumAccess<'de> for TaggedEnumAcc<'a, R> {
	type Error = Error;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let variant = seed.deserialize(&mut *self.de)?;
		Ok((variant, self))
	}
}

impl<'a, 'de, R: Read<'de>> VariantAccess<'de> for TaggedEnumAcc<'a, R> {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		let next = self.de.peek_line()?.ok_or(Error::Eof)?;
		if next != b'[' && next != b'{' {
			return Err(Error::ExpectedSeq(next as char));
		}

		self.de.indent += 1;

		let mut variant = TaggedVariant::new(self.de);
		let val = seed.deserialize(&mut variant)?;

		if !variant.is_map {
			let peek = self.de.peek_any().ok_or(Error::Eof)?;
			self.de.read.discard();

			if peek != b']' {
				return Err(Error::ExpectedSeqEnd(peek as char));
			}
		}

		self.de.indent -= 1;
		Ok(val)
	}

	fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let _ = self.de.peek_line()?.ok_or(Error::Eof)?;
		self.de.deserialize_tuple(len, visitor)
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let _ = self.de.peek_line()?.ok_or(Error::Eof)?;
		self.de.deserialize_map(visitor)
	}
}

struct TaggedVariant<'a, R> {
	de: &'a mut Deserializer<R>,
	bracket_assert: bool,
	is_map: bool,
}

impl<'a, 'de, R: Read<'de>> TaggedVariant<'a, R> {
	fn new(de: &'a mut Deserializer<R>) -> Self {
		TaggedVariant {
			de,
			bracket_assert: false,
			is_map: false,
		}
	}

	fn assert_bracket(&mut self) -> Result<(), Error> {
		if self.bracket_assert {
			return Ok(());
		}

		let peek = self.de.read.next().ok_or(Error::Eof)?;
		if peek != b'[' {
			Err(Error::ExpectedSeq(peek as char))
		} else {
			self.bracket_assert = true;
			let _ = self.de.peek_any().ok_or(Error::Eof);

			Ok(())
		}
	}
}

impl<'a, 'de, R: Read<'de>> serde::de::Deserializer<'de> for &mut TaggedVariant<'a, R> {
	type Error = Error;

	#[allow(unused_variables)]
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_u8(visitor)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_u16(visitor)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_u32(visitor)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_u64(visitor)
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_bool(visitor)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_i8(visitor)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_i16(visitor)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_i32(visitor)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_i64(visitor)
	}

	fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_i128(visitor)
	}

	fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_u128(visitor)
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_f32(visitor)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_f64(visitor)
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_char(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_str(visitor)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	#[allow(unused_variables)]
	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.deserialize_bytes(visitor)
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		visitor.visit_some(self)
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_unit(visitor)
	}

	fn deserialize_unit_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_unit_struct(name, visitor)
	}

	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;

		let acc = SeqAcc::new(self.de);
		let val = visitor.visit_seq(acc)?;

		let peek = self.de.peek_any().ok_or(Error::Eof)?;
		if peek != b']' {
			return Err(Error::ExpectedSeqEnd(peek as char));
		}

		Ok(val)
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.deserialize_seq(visitor)
	}

	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		_len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.de.read.peek().ok_or(Error::Eof)?;
		if peek == b'[' {
			self.de.read.discard();
		} else if peek == b'{' {
			self.is_map = true;
		}

		self.de.deserialize_map(visitor)
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}

	#[allow(unused_variables)]
	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		self.de.deserialize_identifier(visitor)
	}

	forward_to_deserialize_any! { ignored_any }
}
