use super::{fake::FakeStringDeserializer, unit::TaggedUnitEnumAcc};
use crate::{
	de::{
		access::SeqAcc,
		add_span,
		map::MapKey,
		read::{Read, Ref},
	},
	error::ErrorCode,
	Deserializer, Error,
};
use serde_core::de::{Deserializer as _, EnumAccess, VariantAccess};
use std::borrow::Cow;

pub struct TaggedEnumKeyAcc<'a, 'b, 'de, R> {
	map_key: &'a mut MapKey<'b, R>,
	string: Option<Cow<'de, str>>,
}

impl<'a, 'b, 'de, R: Read<'de>> TaggedEnumKeyAcc<'a, 'b, 'de, R> {
	pub fn new(map_key: &'a mut MapKey<'b, R>) -> Self {
		TaggedEnumKeyAcc {
			map_key,
			string: None,
		}
	}

	pub fn with_tag(map_key: &'a mut MapKey<'b, R>, string: Cow<'de, str>) -> Self {
		TaggedEnumKeyAcc {
			map_key,
			string: Some(string),
		}
	}
}

impl<'de, R: Read<'de>> EnumAccess<'de> for TaggedEnumKeyAcc<'_, '_, 'de, R> {
	type Error = Error;
	type Variant = Self;

	fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde_core::de::DeserializeSeed<'de>,
	{
		if let Some(string) = self.string.take() {
			let fake = FakeStringDeserializer::new(string);
			let variant = seed.deserialize(fake)?;
			Ok((variant, self))
		} else {
			let variant = seed.deserialize(&mut *self.map_key)?;
			Ok((variant, self))
		}
	}
}

impl<'de, R: Read<'de>> VariantAccess<'de> for TaggedEnumKeyAcc<'_, '_, 'de, R> {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde_core::de::DeserializeSeed<'de>,
	{
		let next = self.map_key.de.peek_line()?.ok_or(Error::EOF)?;
		if next != '[' {
			let point = self.map_key.de.read.position();
			let code = ErrorCode::ExpectedSeq(next);
			return Err(Error::with_point(code, point));
		}
		self.map_key.de.read.discard();

		self.map_key.de.indent += 1;

		self.map_key.de.peek_any().ok_or(Error::EOF)?;
		let variant = TaggedKey::new(&mut *self.map_key.de);
		let val = seed.deserialize(variant)?;

		self.map_key.de.discard_commata();
		let peek = self.map_key.de.peek_any().ok_or(Error::EOF)?;
		if peek != ']' {
			let point = self.map_key.de.read.position();
			let code = ErrorCode::ExpectedSeqEnd(next);
			return Err(Error::with_point(code, point));
		}
		self.map_key.de.read.discard();

		self.map_key.de.indent -= 1;

		Ok(val)
	}

	fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		let _ = self.map_key.de.peek_line()?.ok_or(Error::EOF)?;
		self.map_key.deserialize_tuple(len, visitor)
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		Err(Error::new(ErrorCode::UnsupportedMapKey("struct")))
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

impl<'de, R: Read<'de>> serde_core::de::Deserializer<'de> for TaggedKey<'_, R> {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		// see the comment in TaggedValue::deserialize_any
		self.de.deserialize_any(visitor)
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_bool(visitor)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_i8(visitor)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_i16(visitor)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_i32(visitor)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_i64(visitor)
	}

	fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_i128(visitor)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_u8(visitor)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_u16(visitor)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_u32(visitor)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_u64(visitor)
	}

	fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_u128(visitor)
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_f32(visitor)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_f64(visitor)
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_char(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_str(visitor)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		let peek = self.de.peek_any().ok_or(Error::EOF)?;
		match peek {
			'"' | '\'' => {
				let (r#ref, sp) = self.de.str_bytes()?;
				match r#ref {
					Ref::Borrow(b) => visitor.visit_borrowed_bytes(b).map_err(|e| add_span(e, sp)),
					Ref::Scratch(s) => visitor.visit_bytes(s).map_err(|e| add_span(e, sp)),
				}
			}
			'0'..='9' => self.deserialize_seq(visitor),
			']' => visitor.visit_borrowed_bytes(&[]),
			_ => {
				let point = self.de.read.position();
				let code = ErrorCode::ExpectedBytes(peek);
				Err(Error::with_point(code, point))
			}
		}
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.deserialize_bytes(visitor)
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		visitor.visit_some(self)
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_unit(visitor)
	}

	fn deserialize_unit_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_unit_struct(name, visitor)
	}

	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		let acc = SeqAcc::new(self.de);
		let val = visitor.visit_seq(acc)?;

		self.de.discard_commata();

		Ok(val)
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
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
		V: serde_core::de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
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
		V: serde_core::de::Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}

	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		let peek = self.de.read.peek().ok_or(Error::EOF)?;
		if peek == '"' || peek == '\'' {
			let acc = TaggedUnitEnumAcc::new(&mut *self.de);
			visitor.visit_enum(acc)
		} else {
			let point = self.de.read.position();
			let code = ErrorCode::ExpectedEnum(peek);
			Err(Error::with_point(code, point))
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		self.de.deserialize_identifier(visitor)
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		// see comment in TaggedValue::deserialize_ignored_any
		self.deserialize_seq(visitor)
	}
}
