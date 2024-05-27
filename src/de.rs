use self::{
	access::TopMapAcc,
	r#enum::TaggedEnumValueAcc,
	read::{Read, Ref, SliceRead, StrRead},
};
use crate::{
	de::access::{MapAcc, SeqAcc},
	error::{Error, ErrorCode},
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
	pub fn from_slice(input: &'de [u8]) -> Self {
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
	/// discard comment
	fn discard_comment(&mut self) -> Option<()> {
		loop {
			let peek = self.read.peek()?;
			self.read.discard();
			if peek == b'\n' {
				break Some(());
			}
		}
	}

	/// peek next character that isn't a whitespace
	fn peek_any(&mut self) -> Option<u8> {
		loop {
			let peek = self.read.peek()?;
			if peek == b'#' {
				self.read.discard();
				self.discard_comment()?;
			} else if read::is_whitespace(peek) {
				self.read.discard();
			} else {
				break Some(peek);
			}
		}
	}

	/// peek next character on the current line, that isn't whitespace.
	///
	/// returns an [`Error::UnexpectedNewline`] error if nothing is found before a new line
	fn peek_line(&mut self) -> Result<Option<u8>, Error> {
		loop {
			let Some(peek) = self.read.peek() else {
				break Ok(None);
			};

			if read::is_whitespace_line(peek) {
				self.read.discard();
			} else if peek == b'\n' || peek == b'#' {
				let point = self.read.position();
				let code = ErrorCode::UnexpectedNewline;
				break Err(Error::with_point(code, point));
			} else {
				break Ok(Some(peek));
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
				break Ok(None);
			};

			if read::is_whitespace_line(peek) {
				self.read.discard()
			} else if peek == b'#' {
				is_newline = true;
				self.read.discard();

				if self.discard_comment().is_none() {
					break Ok(None);
				};
			} else if peek == b'\n' {
				is_newline = true;
				self.read.discard();
			} else if is_newline {
				break Ok(Some(peek));
			} else {
				let point = self.read.position();
				let code = ErrorCode::ExpectedNewline(peek as char);
				break Err(Error::with_point(code, point));
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
			let peek = self.read.peek().ok_or(Error::new(ErrorCode::Eof))?;

			let point = self.read.position();
			let code = ErrorCode::ExpectedNumeric(peek as char);
			Err(Error::with_point(code, point))
		} else {
			Ok(r#ref)
		}
	}

	fn word<'s>(&'s mut self) -> Result<Ref<'de, 's, str>, Error> {
		self.scratch.clear();
		self.read.word(&mut self.scratch)
	}

	fn str<'s>(&'s mut self) -> Result<Ref<'de, 's, str>, Error> {
		let peek = self.read.peek().ok_or(Error::new(ErrorCode::Eof))?;
		if peek != b'"' && peek != b'\'' {
			let point = self.read.position();
			let code = ErrorCode::ExpectedQuote(peek as char);
			return Err(Error::with_point(code, point));
		}

		self.scratch.clear();
		self.read.str(&mut self.scratch)
	}

	fn str_bytes<'s>(&'s mut self) -> Result<Ref<'de, 's, [u8]>, Error> {
		self.scratch.clear();
		self.read.str_bytes(&mut self.scratch)
	}
}

impl<'de, R: Read<'de>> serde::de::Deserializer<'de> for &mut Deserializer<R> {
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
		let b = parse_bool(&w)?;
		visitor.visit_bool(b)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<u8>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_u8(n)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<u16>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_u16(n)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<u32>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_u32(n)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<u64>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_u64(n)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<i8>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_i8(n)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<i16>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_i16(n)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<i32>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_i32(n)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = w
			.parse::<i64>()
			.map_err(|_| Error::new(ErrorCode::InvalidNum(w.into())))?;
		visitor.visit_i64(n)
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.deserialize_f64(visitor)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let w = self.num()?;
		let n = parse_f64(&w)?;
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
		let peek = self.read.peek().ok_or(Error::new(ErrorCode::Eof))?;
		match peek {
			b'"' | b'\'' => {
				let r#ref = self.str_bytes()?;
				match r#ref {
					Ref::Borrow(b) => visitor.visit_borrowed_bytes(b),
					Ref::Scratch(s) => visitor.visit_bytes(s),
				}
			}
			b'[' => self.deserialize_seq(visitor),
			_ => {
				let point = self.read.position();
				let code = ErrorCode::ExpectedBytes(peek as char);
				Err(Error::with_point(code, point))
			}
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
		self.indent += 1;

		let next = self.read.peek().ok_or(Error::new(ErrorCode::Eof))?;
		if next != b'[' {
			let point = self.read.position();
			let code = ErrorCode::ExpectedSeq(next as char);
			return Err(Error::with_point(code, point));
		}
		self.read.discard();

		let seq = SeqAcc::new(self);
		let val = visitor.visit_seq(seq)?;

		self.discard_commata();
		let next = self.peek_any().ok_or(Error::new(ErrorCode::Eof))?;

		if next != b']' {
			let point = self.read.position();
			let code = ErrorCode::ExpectedSeqEnd(next as char);
			return Err(Error::with_point(code, point));
		}
		self.read.discard();

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
			let peek = peek.ok_or(Error::new(ErrorCode::Eof))?;

			let point = self.read.position();
			let code = ErrorCode::ExpectedMap(peek as char);
			return Err(Error::with_point(code, point));
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
		let peek = self.read.peek().ok_or(Error::new(ErrorCode::Eof))?;

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
		let peek = self.read.peek().ok_or(Error::new(ErrorCode::Eof))?;
		if peek == b'"' || peek == b'\'' {
			return self.deserialize_str(visitor);
		} else if !peek.is_ascii_alphabetic() {
			let point = self.read.position();
			let code = ErrorCode::ExpectedAlphabetic(peek as char);
			return Err(Error::with_point(code, point));
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

fn parse_bool(word: &str) -> Result<bool, Error> {
	if word.eq_ignore_ascii_case("true") {
		Ok(true)
	} else if word.eq_ignore_ascii_case("false") {
		Ok(false)
	} else {
		let code = ErrorCode::InvalidBool(word.to_owned());
		Err(Error::new(code))
	}
}

fn parse_f64(num: &str) -> Result<f64, Error> {
	let stripped = if let Some(stripped) = num.strip_prefix('+') {
		if stripped.starts_with(['+', '-']) {
			let code = ErrorCode::InvalidNum(num.to_owned());
			return Err(Error::new(code));
		}
		stripped
	} else {
		num
	};

	if stripped.eq_ignore_ascii_case(".inf") {
		Ok(f64::INFINITY)
	} else if stripped.eq_ignore_ascii_case("-.inf") {
		Ok(f64::NEG_INFINITY)
	} else if stripped.eq_ignore_ascii_case(".nan") {
		let code = ErrorCode::UnsupportedNaN;
		Err(Error::new(code))
	} else if let Ok(float) = stripped.parse::<f64>() {
		if float.is_finite() {
			Ok(float)
		} else {
			unreachable!()
		}
	} else {
		let code = ErrorCode::InvalidNum(num.to_owned());
		Err(Error::new(code))
	}
}
