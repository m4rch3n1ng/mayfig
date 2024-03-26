use self::{
	access::{EnumAcc, MapAcc, SeqAcc, TopMapAcc, UnitEnumAcc},
	read::{IoRead, Read, Ref, StrRead},
};
use crate::error::Err;

mod access;
mod read;

pub struct Deserializer<R> {
	read: R,
	indent: usize,
	scratch: Vec<u8>,
}

impl<'de, R: Read<'de>> Deserializer<R> {
	fn new(read: R) -> Self {
		Deserializer {
			read,
			indent: 0,
			scratch: Vec::new(),
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

impl<R: std::io::Read> Deserializer<IoRead<R>> {
	fn from_reader(read: R) -> Self {
		let read = IoRead::new(read);
		Deserializer::new(read)
	}
}

pub fn from_reader<R, T>(read: R) -> Result<T, Err>
where
	R: std::io::Read,
	T: serde::de::DeserializeOwned,
{
	let mut deserializer = Deserializer::from_reader(read);
	let t = T::deserialize(&mut deserializer);
	t
}

impl<'de, R: Read<'de>> Deserializer<R> {
	fn peek_whitespace(&mut self) -> Result<Option<u8>, Err> {
		loop {
			let Some(peek) = self.read.peek()? else {
				return Ok(None);
			};

			if read::is_whitespace(peek) {
				self.read.discard();
			} else {
				return Ok(Some(peek));
			}
		}
	}

	fn next_whitespace(&mut self) -> Result<Option<u8>, Err> {
		loop {
			let Some(next) = self.read.next()? else {
				return Ok(None);
			};

			if !read::is_whitespace(next) {
				return Ok(Some(next));
			}
		}
	}

	fn discard_all(&mut self, ch: u8) {
		while self.peek_whitespace().is_ok_and(|peek| peek == Some(ch)) {
			self.read.discard();
		}
	}

	fn num<'s>(&'s mut self) -> Result<Ref<'de, 's>, Err> {
		self.peek_whitespace()?.ok_or(Err::Eof)?;

		self.scratch.clear();
		self.read.num(&mut self.scratch)
	}

	fn word<'s>(&'s mut self) -> Result<Ref<'de, 's>, Err> {
		self.peek_whitespace()?.ok_or(Err::Eof)?;

		self.scratch.clear();
		self.read.word(&mut self.scratch)
	}

	fn str<'s>(&'s mut self) -> Result<Ref<'de, 's>, Err> {
		self.peek_whitespace()?;

		self.scratch.clear();
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
		let peek = self.peek_whitespace()?.ok_or(Err::Eof)?;
		if self.indent == 0 || peek == b'{' {
			self.deserialize_map(visitor)
		} else if peek == b'[' {
			self.deserialize_seq(visitor)
		} else if let b'0'..=b'9' | b'.' | b'-' = peek {
			self.deserialize_number(visitor)
		} else if peek == b'"' || peek == b'\'' {
			self.deserialize_str(visitor)
		} else {
			let word = self.word()?;
			if let Ok(b) = parse_bool(&word) {
				visitor.visit_bool(b)
			} else if let Ok(f) = word.parse::<f64>() {
				visitor.visit_f64(f)
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
		let b = parse_bool(&w)?;
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
		self.deserialize_str(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_whitespace()?.ok_or(Err::Eof)?;

		if let b'"' | b'\'' = peek {
			let string = self.str()?;
			match string {
				Ref::Borrow(b) => visitor.visit_borrowed_str(b),
				Ref::Scratch(s) => visitor.visit_str(s),
			}
		} else {
			Err(Err::Expected('"', char::from(peek)))
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

		let next = self.next_whitespace()?.ok_or(Err::Eof)?;
		if next != b'[' {
			return Err(Err::Expected('[', char::from(next)));
		}

		let acc = SeqAcc::new(self);
		let val = visitor.visit_seq(acc)?;

		let peek = self.next_whitespace()?.ok_or(Err::Eof)?;
		if peek != b']' {
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
		let peek = self.peek_whitespace()?;
		let val = if let Some(b'{') = peek {
			self.indent += 1;
			self.read.discard();

			let acc = MapAcc::new(self);
			let val = visitor.visit_map(acc)?;

			self.indent -= 1;

			Ok(val)
		} else if self.indent != 0 {
			if let Some(peek) = peek {
				Err(Err::Expected('{', char::from(peek)))
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
		let peek = self.peek_whitespace()?.ok_or(Err::Eof)?;
		if peek == b'{' {
			self.read.discard();

			let acc = EnumAcc::new(self);
			let val = visitor.visit_enum(acc)?;

			let next = self.next_whitespace()?.ok_or(Err::Eof)?;
			if next == b'}' {
				Ok(val)
			} else {
				Err(Err::Expected('}', char::from(next)))
			}
		} else if peek == b'"' || peek == b'\'' {
			let acc = UnitEnumAcc::new(self);
			visitor.visit_enum(acc)
		} else {
			Err(Err::UnexpectedChar(char::from(peek), "[enum]"))
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_whitespace()?.ok_or(Err::Eof)?;
		if peek == b'"' || peek == b'\'' {
			return self.deserialize_str(visitor);
		} else if !peek.is_ascii_alphabetic() {
			return Err(Err::UnexpectedChar(
				char::from(peek),
				"[ident] alphanumeric",
			));
		}

		let ident = self.word()?;
		match ident {
			Ref::Borrow(b) => visitor.visit_borrowed_str(b),
			Ref::Scratch(s) => visitor.visit_str(s),
		}
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
