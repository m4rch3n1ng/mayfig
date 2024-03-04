use crate::error::Err;
use serde::{
	de::{EnumAccess, MapAccess, SeqAccess, VariantAccess},
	forward_to_deserialize_any,
};

struct Read<'de> {
	input: &'de str,
}

impl<'de> Read<'de> {
	fn peek(&self) -> Result<char, Err> {
		self.input.chars().next().ok_or(Err::Eof)
	}

	fn next(&mut self) -> Result<char, Err> {
		let char = self.input.chars().next().ok_or(Err::Eof)?;
		let len = char.len_utf8();
		self.input = &self.input[len..];
		Ok(char)
	}

	fn discard(&mut self) {
		let _ = self.next();
	}

	fn num(&mut self) -> Result<&'de str, Err> {
		let mut one = &self.input[0..0];
		let mut two = self.input;

		if let Some('-') = two.chars().next() {
			let len = '-'.len_utf8();

			one = &self.input[..len];
			two = &self.input[len..];
		}

		loop {
			let Some(nxt) = two.chars().next() else {
				break;
			};

			if let '0'..='9' | '.' = nxt {
				let len = one.len() + nxt.len_utf8();

				one = &self.input[..len];
				two = &self.input[len..];
			} else if nxt.is_ascii_whitespace() || nxt.is_ascii_punctuation() {
				break;
			} else {
				return Err(Err::UnexpectedChar(nxt, "[num] numeric"));
			}
		}

		self.input = two;
		Ok(one)
	}

	fn word(&mut self) -> Result<&'de str, Err> {
		let mut one = &self.input[0..0];
		let mut two = self.input;

		loop {
			let Some(nxt) = two.chars().next() else {
				break;
			};

			if nxt.is_alphanumeric() || nxt == '_' {
				let len = one.len() + nxt.len_utf8();

				one = &self.input[..len];
				two = &self.input[len..];
			} else if nxt.is_ascii_whitespace() || nxt.is_ascii_punctuation() {
				break;
			} else {
				return Err(Err::UnexpectedChar(nxt, "[word] alphanumeric"));
			}
		}

		self.input = two;
		Ok(one)
	}
}

pub struct Deserializer<'de> {
	read: Read<'de>,
	indent: usize,
}

impl<'de> Deserializer<'de> {
	#[allow(clippy::should_implement_trait)]
	pub fn from_str(input: &'de str) -> Self {
		let read = Read { input };
		Deserializer { read, indent: 0 }
	}
}

pub fn from_str<'a, T>(input: &'a str) -> Result<T, Err>
where
	T: serde::de::Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_str(input);
	let t = T::deserialize(&mut deserializer);
	t
}

impl<'de> Deserializer<'de> {
	fn peek_whitespace(&mut self) -> Result<char, Err> {
		loop {
			match self.read.peek()? {
				' ' | '\t' | '\r' | '\n' => self.read.discard(),
				c => return Ok(c),
			}
		}
	}

	fn next_whitespace(&mut self) -> Result<char, Err> {
		loop {
			match self.read.next()? {
				' ' | '\t' | '\r' | '\n' => (),
				c => return Ok(c),
			}
		}
	}

	fn discard_all(&mut self, ch: char) {
		while self.peek_whitespace().is_ok_and(|peek| peek == ch) {
			self.read.discard();
		}
	}

	fn num(&mut self) -> Result<&'de str, Err> {
		self.peek_whitespace()?;
		self.read.num()
	}

	fn word(&mut self) -> Result<&'de str, Err> {
		self.peek_whitespace()?;
		self.read.word()
	}

	fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value, Err>
	where
		V: serde::de::Visitor<'any>,
	{
		let w = self.num()?;
		if w.contains('.') {
			let n = w.parse::<f64>().map_err(|_| Err::InvalidNum(w.into()))?;
			visitor.visit_f64(n)
		} else if w.starts_with('-') {
			let n = w.parse::<i64>().map_err(|_| Err::InvalidNum(w.into()))?;
			visitor.visit_i64(n)
		} else {
			let n = w.parse::<u64>().map_err(|_| Err::InvalidNum(w.into()))?;
			visitor.visit_u64(n)
		}
	}
}

#[allow(unused_variables)]
impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
	type Error = Err;
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_whitespace()?;
		if self.indent == 0 || peek == '{' {
			self.deserialize_map(visitor)
		} else if peek.is_alphabetic() {
			self.deserialize_identifier(visitor)
		} else if peek.is_numeric() || peek == '-' {
			self.deserialize_number(visitor)
		} else if peek == '"' {
			self.deserialize_str(visitor)
		} else {
			todo!("any {}", peek)
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i8>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_i8(n)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i16>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_i16(n)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i32>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_i32(n)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i64>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_i64(n)
	}

	fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u8>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_u8(n)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u16>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_u16(n)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u32>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_u32(n)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u64>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_u64(n)
	}

	fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<f32>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_f32(n)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<f64>().map_err(|_| Err::InvalidNum(w.into()))?;
		visitor.visit_f64(n)
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!("str")
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
		todo!()
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_unit_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_newtype_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.indent += 1;

		let next = self.next_whitespace()?;
		if next != '[' {
			return Err(Err::Expected('[', next));
		}

		let acc = SeqAcc::new(self);
		let val = visitor.visit_seq(acc)?;

		self.indent -= 1;

		Ok(val)
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
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
		todo!()
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_whitespace()?;
		let val = if peek == '{' {
			self.indent += 1;
			self.read.discard();

			let acc = MapAcc::new(self);
			let val = visitor.visit_map(acc)?;

			self.indent -= 1;

			Ok(val)
		} else if self.indent != 0 {
			Err(Err::Expected('{', peek))
		} else {
			self.indent += 1;

			let acc = TopMapAcc::new(self);
			let val = visitor.visit_map(acc)?;

			self.indent -= 1;

			Ok(val)
		};

		val
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
		self.deserialize_map(visitor)
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
		let peek = self.peek_whitespace()?;
		if peek == '{' {
			self.read.discard();

			let acc = EnumAcc::new(self);
			let val = visitor.visit_enum(acc)?;

			let next = self.next_whitespace()?;
			if next == '}' {
				Ok(val)
			} else {
				Err(Err::Expected('}', next))
			}
		} else if peek == '"' {
			todo!("unit enum")
		} else {
			Err(Err::UnexpectedChar(peek, "[enum]"))
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_whitespace()?;
		if peek == '"' || peek == '\'' {
			return self.deserialize_str(visitor);
		} else if !peek.is_ascii_alphabetic() {
			return Err(Err::UnexpectedChar(peek, "[ident] alphanumeric"));
		}

		let ident = self.word()?;
		visitor.visit_borrowed_str(ident)
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}
}

struct TopMapAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> TopMapAcc<'a, 'de> {
	fn new(de: &'a mut Deserializer<'de>) -> Self {
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

struct MapAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAcc<'a, 'de> {
	fn new(de: &'a mut Deserializer<'de>) -> Self {
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

struct SeqAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SeqAcc<'a, 'de> {
	fn new(de: &'a mut Deserializer<'de>) -> Self {
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

struct EnumAcc<'a, 'de> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> EnumAcc<'a, 'de> {
	fn new(de: &'a mut Deserializer<'de>) -> Self {
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
