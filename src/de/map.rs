use super::{
	access::SeqAcc,
	read::{Read, Ref},
};
use crate::{error::Error, Deserializer};
use serde::{
	de::{EnumAccess, VariantAccess},
	forward_to_deserialize_any, Deserializer as _,
};

pub struct MapKey<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> MapKey<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		MapKey { de }
	}
}

pub struct TaggedEnumKeyAcc<'a, 'b, R> {
	map_key: &'a mut MapKey<'b, R>,
}

impl<'a, 'b, 'de, R: Read<'de>> TaggedEnumKeyAcc<'a, 'b, R> {
	pub fn new(map_key: &'a mut MapKey<'b, R>) -> Self {
		TaggedEnumKeyAcc { map_key }
	}
}

impl<'a, 'b, 'de, R: Read<'de>> EnumAccess<'de> for TaggedEnumKeyAcc<'a, 'b, R> {
	type Error = Error;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let variant = seed.deserialize(&mut *self.map_key)?;
		Ok((variant, self))
	}
}

impl<'a, 'b, 'de, R: Read<'de>> VariantAccess<'de> for TaggedEnumKeyAcc<'a, 'b, R> {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		let next = self.map_key.de.peek_line()?.ok_or(Error::Eof)?;
		self.map_key.de.read.discard();
		if next != b'[' {
			return Err(Error::ExpectedSeq(next as char));
		}

		self.map_key.de.indent += 1;

		self.map_key.de.peek_any().ok_or(Error::Eof)?;
		let variant = TaggedKey::new(&mut *self.map_key.de);
		let val = seed.deserialize(variant)?;

		let peek = self.map_key.de.peek_any().ok_or(Error::Eof)?;
		self.map_key.de.read.discard();
		if peek != b']' {
			return Err(Error::ExpectedSeqEnd(peek as char));
		}

		self.map_key.de.indent -= 1;

		Ok(val)
	}

	fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let _ = self.map_key.de.peek_line()?.ok_or(Error::Eof)?;
		self.map_key.deserialize_tuple(len, visitor)
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::UnsupportedMapKey("struct"))
	}
}

struct TaggedKey<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> TaggedKey<'a, R> {
	fn new(de: &'a mut Deserializer<R>) -> Self {
		TaggedKey { de }
	}
}

impl<'a, 'de, R: Read<'de>> serde::de::Deserializer<'de> for TaggedKey<'a, R> {
	type Error = Error;

	#[allow(unused_variables)]
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
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

	fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_i128(visitor)
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

	fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_u128(visitor)
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
		self.de.deserialize_str(visitor)
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
		let peek = self.de.peek_any().ok_or(Error::Eof)?;
		match peek {
			b'"' | b'\'' => {
				let r#ref = self.de.str_bytes()?;
				match r#ref {
					Ref::Borrow(b) => visitor.visit_borrowed_bytes(b),
					Ref::Scratch(s) => visitor.visit_bytes(s),
				}
			}
			b'0'..=b'9' => self.deserialize_seq(visitor),
			b']' => visitor.visit_borrowed_bytes(&[]),
			_ => Err(Error::ExpectedBytes(peek as char)),
		}
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_bytes(visitor)
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
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
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
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
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
		self.de.deserialize_identifier(visitor)
	}

	forward_to_deserialize_any! { ignored_any }
}

impl<'a, 'de, R: Read<'de>> serde::de::Deserializer<'de> for &mut MapKey<'a, R> {
	type Error = Error;

	#[allow(unused_variables)]
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_bool(visitor)
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

	fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_u128(visitor)
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

	fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_i128(visitor)
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
		self.deserialize_str(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_identifier(visitor)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::UnsupportedMapKey("bytes"))
	}

	fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::UnsupportedMapKey("bytes"))
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		// what the fuck are you even doing
		visitor.visit_some(self)
	}

	fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::UnsupportedUnit)
	}

	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::UnsupportedUnit)
	}

	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
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
		self.de.deserialize_seq(visitor)
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
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
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::UnsupportedMapKey("map"))
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::UnsupportedMapKey("struct"))
	}

	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.de.read.peek().ok_or(Error::Eof)?;

		if peek == b'{' {
			Err(Error::UnsupportedMapKey("struct"))
		} else if peek.is_ascii_alphabetic() || peek == b'"' || peek == b'\'' {
			let acc = TaggedEnumKeyAcc::new(self);
			visitor.visit_enum(acc)
		} else {
			todo!()
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.de.deserialize_identifier(visitor)
	}

	forward_to_deserialize_any! { ignored_any }
}
