use self::{
	access::TopMapAcc,
	r#enum::TaggedEnumValueAcc,
	read::{Read, Ref, SliceRead, StrRead},
};
use crate::{
	de::access::{MapAcc, SeqAcc},
	error::Error,
};

mod access;
mod r#enum;
mod map;
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

pub fn from_str<'a, T>(input: &'a str) -> Result<T, Error>
where
	T: serde::de::Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_str(input);
	T::deserialize(&mut deserializer)
}

impl<'de> Deserializer<SliceRead<'de>> {
	fn from_slice(input: &'de [u8]) -> Self {
		let read = SliceRead::new(input);
		Deserializer::new(read)
	}
}

pub fn from_slice<'a, T>(input: &'a [u8]) -> Result<T, Error>
where
	T: serde::de::Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_slice(input);
	T::deserialize(&mut deserializer)
}

impl<'de, R: Read<'de>> Deserializer<R> {
	/// peek next character that isn't a whitespace
	fn peek_any(&mut self) -> Option<u8> {
		loop {
			let peek = self.read.peek()?;
			if read::is_whitespace(peek) {
				self.read.discard();
			} else {
				return Some(peek);
			}
		}
	}

	/// peek next character on the current line, that isn't whitespace.
	///
	/// returns an [`Error::UnexpectedNewline`] error if nothing is found before a new line
	fn peek_line(&mut self) -> Result<Option<u8>, Error> {
		loop {
			let Some(peek) = self.read.peek() else {
				return Ok(None);
			};

			if read::is_whitespace_line(peek) {
				self.read.discard();
			} else if peek == b'\n' {
				return Err(Error::UnexpectedNewline);
			} else {
				return Ok(Some(peek));
			}
		}
	}

	/// peek next character on the next line, that isn't whitespace.
	///
	/// returns an [`Error::ExpectedNewline`] error if something was found before the linebreak
	fn peek_newline(&mut self) -> Result<Option<u8>, Error> {
		let mut is_newline = false;
		loop {
			let Some(peek) = self.read.peek() else {
				return Ok(None);
			};

			if read::is_whitespace_line(peek) {
				self.read.discard()
			} else if peek == b'\n' {
				is_newline = true;
				self.read.discard();
			} else if is_newline {
				return Ok(Some(peek));
			} else {
				return Err(Error::ExpectedNewline(peek as char));
			}
		}
	}

	fn discard_commata(&mut self) {
		while self.peek_any().is_some_and(|peek| peek == b',') {
			self.read.discard();
		}
	}

	fn num<'s>(&'s mut self) -> Result<Ref<'de, 's, str>, Error> {
		self.scratch.clear();

		let r#ref = self.read.num(&mut self.scratch)?;
		if r#ref.is_empty() {
			let peek = self.read.peek().ok_or(Error::Eof)?;
			Err(Error::ExpectedNumeric(peek as char))
		} else {
			Ok(r#ref)
		}
	}

	fn word<'s>(&'s mut self) -> Result<Ref<'de, 's, str>, Error> {
		self.scratch.clear();
		self.read.word(&mut self.scratch)
	}

	fn str<'s>(&'s mut self) -> Result<Ref<'de, 's, str>, Error> {
		let peek = self.read.peek().ok_or(Error::Eof)?;
		if peek != b'"' && peek != b'\'' {
			return Err(Error::ExpectedQuote(peek as char));
		}

		self.scratch.clear();
		self.read.str(&mut self.scratch)
	}

	fn str_bytes<'s>(&'s mut self) -> Result<Ref<'de, 's, [u8]>, Error> {
		self.scratch.clear();
		self.read.str_bytes(&mut self.scratch)
	}
}

impl<'de, R: Read<'de>> serde::Deserializer<'de> for &mut Deserializer<R> {
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
		let w = self.word()?;
		let b = read::parse_bool(&w)?;
		visitor.visit_bool(b)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u8>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_u8(n)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u16>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_u16(n)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u32>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_u32(n)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<u64>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_u64(n)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i8>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_i8(n)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i16>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_i16(n)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i32>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_i32(n)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<i64>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_i64(n)
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<f32>().map_err(|_| Error::InvalidNum(w.into()))?;
		visitor.visit_f32(n)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w.parse::<f64>().map_err(|_| Error::InvalidNum(w.into()))?;
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
		let r#str = self.str()?;
		match r#str {
			Ref::Borrow(b) => visitor.visit_borrowed_str(b),
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
		let peek = self.read.peek().ok_or(Error::Eof)?;
		match peek {
			b'"' | b'\'' => {
				let r#ref = self.str_bytes()?;
				match r#ref {
					Ref::Borrow(b) => visitor.visit_borrowed_bytes(b),
					Ref::Scratch(s) => visitor.visit_bytes(s),
				}
			}
			b'[' => self.deserialize_seq(visitor),
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
		self.indent += 1;

		let next = self.read.next().ok_or(Error::Eof)?;
		if next != b'[' {
			return Err(Error::ExpectedSeq(next as char));
		}

		let seq = SeqAcc::new(self);
		let val = visitor.visit_seq(seq)?;

		self.discard_commata();
		let next = self.peek_any().ok_or(Error::Eof)?;
		self.read.discard();

		if next != b']' {
			return Err(Error::ExpectedSeqEnd(next as char));
		}

		self.indent -= 1;

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
		let peek = self.peek_any();
		let value = if let Some(b'{') = peek {
			self.indent += 1;
			self.read.discard();

			let map_acc = MapAcc::new(self);
			let val = visitor.visit_map(map_acc)?;

			self.indent -= 1;

			Ok(val)
		} else if self.indent != 0 {
			let peek = peek.ok_or(Error::Eof)?;
			return Err(Error::ExpectedMap(peek as char));
		} else {
			self.indent += 1;

			let top_map_acc = TopMapAcc::new(self);
			let val = visitor.visit_map(top_map_acc)?;

			self.indent -= 1;

			Ok(val)
		};

		value
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

	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.read.peek().ok_or(Error::Eof)?;

		if peek == b'{' {
			todo!()
		} else if peek == b'"' || peek == b'\'' {
			let acc = TaggedEnumValueAcc::new(self);
			visitor.visit_enum(acc)
		} else {
			todo!()
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.read.peek().ok_or(Error::Eof)?;
		if peek == b'"' || peek == b'\'' {
			return self.deserialize_str(visitor);
		} else if !peek.is_ascii_alphabetic() {
			return Err(Error::ExpectedAlphabetic(peek as char));
		}

		let identifier = self.word()?;
		match identifier {
			Ref::Borrow(b) => visitor.visit_borrowed_str(b),
			Ref::Scratch(s) => visitor.visit_str(s),
		}
	}

	#[allow(unused_variables)]
	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}
}
