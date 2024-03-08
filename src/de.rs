use self::{
	access::{EnumAcc, MapAcc, SeqAcc, TopMapAcc, UnitEnumAcc},
	read::{Read, Ref, StrRead},
};
use crate::error::Err;

mod access;
mod read;

pub struct Deserializer<R> {
	read: R,
	indent: usize,
	scratch: String,
}

impl<'de, R: Read<'de>> Deserializer<R> {
	fn new(read: R) -> Self {
		Deserializer {
			read,
			indent: 0,
			scratch: String::new(),
		}
	}
}

impl<'de> Deserializer<StrRead<'de>> {
	#[allow(clippy::should_implement_trait)]
	pub fn from_str(input: &'de str) -> Self {
		let read = StrRead::new(input);
		Deserializer::new(read)
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

impl<'de, R: Read<'de>> Deserializer<R> {
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

	fn str<'s>(&'s mut self) -> Result<Ref<'de, 's>, Err> {
		self.peek_whitespace()?;
		self.read.str(&mut self.scratch)
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
impl<'de, 'a, R: Read<'de>> serde::de::Deserializer<'de> for &'a mut Deserializer<R> {
	type Error = Err;
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_whitespace()?;
		if self.indent == 0 || peek == '{' {
			self.deserialize_map(visitor)
		} else if peek == '[' {
			self.deserialize_seq(visitor)
		} else if peek.is_alphabetic() {
			self.deserialize_identifier(visitor)
		} else if let '0'..='9' | '.' | '-' = peek {
			self.deserialize_number(visitor)
		} else if peek == '"' {
			self.deserialize_str(visitor)
		} else {
			let word = self.word()?;
			if let Ok(b) = parse_bool(word) {
				visitor.visit_bool(b)
			} else {
				Err(Err::UnexpectedWord(word.into()))
			}
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.word()?;
		let b = parse_bool(w)?;
		visitor.visit_bool(b)
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
		let string = self.str()?;
		match string {
			Ref::Borrow(s) => visitor.visit_borrowed_str(s),
			Ref::Scratch(s) => visitor.visit_str(s),
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
		visitor.visit_some(self)
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
		visitor.visit_newtype_struct(self)
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

		let peek = self.next_whitespace()?;
		if peek == ']' {
			self.read.discard();
		} else {
			return Err(Err::ExpectedSeqEnd);
		}

		self.indent -= 1;

		Ok(val)
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
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
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_whitespace();
		let val = if let Ok('{') = peek {
			self.indent += 1;
			self.read.discard();

			let acc = MapAcc::new(self);
			let val = visitor.visit_map(acc)?;

			self.indent -= 1;

			Ok(val)
		} else if self.indent != 0 {
			if let Ok(peek) = peek {
				Err(Err::Expected('{', peek))
			} else {
				Err(Err::Eof)
			}
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
			let acc = UnitEnumAcc::new(self);
			visitor.visit_enum(acc)
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

fn parse_bool(string: &str) -> Result<bool, Err> {
	match string {
		"true" | "on" | "yes" => Ok(true),
		"false" | "off" | "no" => Ok(false),
		_ => Err(Err::InvalidBool(string.to_owned())),
	}
}
