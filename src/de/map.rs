use super::{add_span, r#enum::TaggedEnumKeyAcc, read::Read};
use crate::{
	de::read::Ref,
	error::{Error, ErrorCode},
	Deserializer,
};
use serde::forward_to_deserialize_any;
use std::borrow::Cow;

pub struct MapKey<'a, R> {
	pub(super) de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> MapKey<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		MapKey { de }
	}
}

impl<'de, R: Read<'de>> serde::de::Deserializer<'de> for &mut MapKey<'_, R> {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.de.peek_any().ok_or(Error::EOF)?;
		match peek {
			b'[' => self.deserialize_seq(visitor),
			b'{' => self.deserialize_map(visitor),
			b'0'..=b'9' | b'.' | b'-' | b'+' => self.de.deserialize_number(visitor),
			_ => {
				let (ident, span) = self.de.identifier()?;
				let ident = match ident {
					Ref::Borrow(s) => Cow::Borrowed(s),
					Ref::Scratch(s) => Cow::Owned(s.to_owned()),
				};

				if let Ok(Some(b'[')) = self.de.peek_line() {
					let tagged = TaggedEnumKeyAcc::with_tag(self, ident);
					visitor.visit_enum(tagged)
				} else {
					match ident {
						Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
						Cow::Owned(s) => visitor.visit_string(s),
					}
					.map_err(|e| add_span(e, span))
				}
			}
		}
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
		Err(Error::new(ErrorCode::UnsupportedMapKey("bytes")))
	}

	fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::new(ErrorCode::UnsupportedMapKey("bytes")))
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
		Err(Error::new(ErrorCode::UnsupportedUnit))
	}

	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::new(ErrorCode::UnsupportedUnit))
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
		Err(Error::new(ErrorCode::UnsupportedMapKey("map")))
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
		Err(Error::new(ErrorCode::UnsupportedMapKey("struct")))
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
		let peek = self.de.read.peek().ok_or(Error::EOF)?;
		if peek.is_ascii_alphabetic() || peek == b'"' || peek == b'\'' {
			let acc = TaggedEnumKeyAcc::new(self);
			visitor.visit_enum(acc)
		} else {
			let point = self.de.read.position();
			let code = ErrorCode::ExpectedEnum(self.de.read.peek_char()?);
			Err(Error::with_point(code, point))
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
