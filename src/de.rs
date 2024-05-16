use self::{
	access::TopMapAcc,
	read::{Read, Ref, StrRead},
};
use crate::error::Error;

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

pub fn from_str<'a, T>(input: &'a str) -> Result<T, Error>
where
	T: serde::de::Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_str(input);
	let t = T::deserialize(&mut deserializer);
	t
}

impl<'de, R: Read<'de>> Deserializer<R> {
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
				return Err(Error::ExpectedNewline);
			}
		}
	}

	fn num<'s>(&'s mut self) -> Result<Ref<'de, 's, str>, Error> {
		self.peek_line()?;

		self.scratch.clear();
		self.read.num(&mut self.scratch)
	}

	fn word<'s>(&'s mut self) -> Result<Ref<'de, 's, str>, Error> {
		self.scratch.clear();
		self.read.word(&mut self.scratch)
	}
}

#[allow(unused_variables)]
impl<'de, 'a, R: Read<'de>> serde::Deserializer<'de> for &'a mut Deserializer<R> {
	type Error = Error;

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
		todo!()
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
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
		todo!()
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
		let peek = self.peek_any();
		let value = if let Some(b'{') = peek {
			todo!()
		} else if self.indent != 0 {
			todo!()
		} else {
			self.indent += 1;

			let top_map_acc = TopMapAcc::new(self);
			let value = visitor.visit_map(top_map_acc);

			self.indent -= 1;

			value
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

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}
}
