use crate::error::Err;
use serde::de::{MapAccess, SeqAccess};

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

	// todo nope, that's an awful way to solve that
	fn nth(&self, n: usize) -> Result<char, Err> {
		let str = self.input.get(n..).ok_or(Err::Eof)?;
		let ch = str.chars().next().ok_or(Err::Eof)?;
		Ok(ch)
	}

	fn discard(&mut self) {
		let _ = self.next();
	}

	fn slice(&mut self, end: usize) -> &'de str {
		let slice = &self.input[..end];
		self.input = &self.input[end..];
		slice
	}
}

pub struct Deserializer<'de> {
	read: Read<'de>,
}

impl<'de> Deserializer<'de> {
	#[allow(clippy::should_implement_trait)]
	pub fn from_str(input: &'de str) -> Self {
		let read = Read { input };
		Deserializer { read }
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

	fn word(&mut self) -> Result<&'de str, Err> {
		self.peek_whitespace()?;

		let mut end = 0;
		loop {
			let Ok(nxt) = self.read.nth(end) else {
				break;
			};

			if nxt.is_alphanumeric() {
				end += nxt.len_utf8();
			} else if nxt.is_ascii_whitespace() {
				break;
			} else {
				return Err(Err::UnexpectedChar(nxt, "[word] alphanumeric"));
			}
		}

		let word = self.read.slice(end);
		Ok(word)
	}
}

#[allow(unused_variables)]
impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
	type Error = Err;
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
		todo!()
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
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
		let w = self.word()?;
		let n = w.parse::<u8>().map_err(|_| Err::InvalidNum)?;
		visitor.visit_u8(n)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.word()?;
		let n = w.parse::<u16>().map_err(|_| Err::InvalidNum)?;
		visitor.visit_u16(n)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.word()?;
		let n = w.parse::<u32>().map_err(|_| Err::InvalidNum)?;
		visitor.visit_u32(n)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.word()?;
		let n = w.parse::<u64>().map_err(|_| Err::InvalidNum)?;
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
		todo!()
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
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
		let next = self.next_whitespace()?;
		if next != '[' {
			return Err(Err::Expected('[', next));
		}

		let acc = SeqAcc::new(self);
		let val = visitor.visit_seq(acc)?;
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
		let next = self.next_whitespace()?;
		if next != '{' {
			return Err(Err::Expected('{', next));
		}

		let acc = MapAcc::new(self);
		let val = visitor.visit_map(acc)?;
		Ok(val)
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
		let nxt = self.peek_whitespace()?;
		match nxt {
			'{' => self.deserialize_map(visitor),
			_ => {
				let acc = TopMapAcc::new(self);
				let val = visitor.visit_map(acc);
				val
			}
		}
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
		todo!()
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
			self.de.read.discard();
		}

		seed.deserialize(&mut *self.de).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		let peek = self.de.peek_whitespace()?;
		if peek == '=' {
			self.de.read.discard()
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
		let peek = self.de.peek_whitespace()?;
		if peek == ';' {
			self.de.read.discard();
		} else if peek == '}' {
			self.de.read.discard();
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
			self.de.read.discard()
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
		let peek = self.de.peek_whitespace()?;
		// todo ? multiple ','
		if peek == ',' {
			self.de.read.discard();
		} else if peek == ']' {
			self.de.read.discard();
			return Ok(None);
		}

		seed.deserialize(&mut *self.de).map(Some)
	}
}
