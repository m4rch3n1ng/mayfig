use super::fake::FakeStringDeserializer;
use crate::{
	de::{
		access::SeqAcc,
		read::{Read, Ref},
	},
	error::{Error, ErrorCode},
	Deserializer,
};
use serde::{
	de::{Deserializer as _, EnumAccess, VariantAccess},
	forward_to_deserialize_any,
};

pub struct TaggedEnumValueAcc<'a, R> {
	de: &'a mut Deserializer<R>,
	string: Option<String>,
}

impl<'a, 'de, R: Read<'de>> TaggedEnumValueAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		TaggedEnumValueAcc { de, string: None }
	}

	pub fn with_tag(de: &'a mut Deserializer<R>, string: String) -> Self {
		TaggedEnumValueAcc {
			de,
			string: Some(string),
		}
	}
}

impl<'de, R: Read<'de>> EnumAccess<'de> for TaggedEnumValueAcc<'_, R> {
	type Error = Error;
	type Variant = Self;

	fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		if let Some(string) = self.string.take() {
			let fake = FakeStringDeserializer::new(string);
			let variant = seed.deserialize(fake)?;
			Ok((variant, self))
		} else {
			let variant = seed.deserialize(&mut *self.de)?;
			Ok((variant, self))
		}
	}
}

impl<'de, R: Read<'de>> VariantAccess<'de> for TaggedEnumValueAcc<'_, R> {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		let next = self.de.peek_line()?.ok_or(Error::EOF)?;
		if next != b'[' && next != b'{' {
			let point = self.de.read.position();
			let code = ErrorCode::ExpectedSeq(next as char);
			return Err(Error::with_point(code, point));
		}

		self.de.indent += 1;

		let mut variant = TaggedValue::new(self.de);
		let val = seed.deserialize(&mut variant)?;

		if !variant.is_map {
			self.de.discard_commata();
			let peek = self.de.peek_any().ok_or(Error::EOF)?;

			if peek != b']' {
				let point = self.de.read.position();
				let code = ErrorCode::ExpectedSeqEnd(peek as char);
				return Err(Error::with_point(code, point));
			}

			self.de.read.discard();
		}

		self.de.indent -= 1;
		Ok(val)
	}

	fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let _ = self.de.peek_line()?.ok_or(Error::EOF)?;
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
		let _ = self.de.peek_line()?.ok_or(Error::EOF)?;
		self.de.deserialize_map(visitor)
	}
}

struct TaggedValue<'a, R> {
	de: &'a mut Deserializer<R>,
	bracket_assert: bool,
	is_map: bool,
}

impl<'a, 'de, R: Read<'de>> TaggedValue<'a, R> {
	fn new(de: &'a mut Deserializer<R>) -> Self {
		TaggedValue {
			de,
			bracket_assert: false,
			is_map: false,
		}
	}

	fn assert_bracket(&mut self) -> Result<(), Error> {
		if self.bracket_assert {
			return Ok(());
		}

		let peek = self.de.read.peek().ok_or(Error::EOF)?;
		if peek != b'[' {
			let point = self.de.read.position();
			let code = ErrorCode::ExpectedSeq(peek as char);
			Err(Error::with_point(code, point))
		} else {
			self.de.read.discard();

			self.bracket_assert = true;
			let _ = self.de.peek_any().ok_or(Error::EOF);

			Ok(())
		}
	}
}

impl<'de, R: Read<'de>> serde::de::Deserializer<'de> for &mut TaggedValue<'_, R> {
	type Error = Error;

	#[expect(unused_variables)]
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

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.assert_bracket()?;
		let peek = self.de.peek_any().ok_or(Error::EOF)?;
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
			_ => {
				let point = self.de.read.position();
				let code = ErrorCode::ExpectedBytes(peek as char);
				Err(Error::with_point(code, point))
			}
		}
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

		self.de.discard_commata();

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
		let peek = self.de.read.peek().ok_or(Error::EOF)?;
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

	#[expect(unused_variables)]
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
