//! deserialize mayfig into a rust data structure.

use self::{
	access::TopMapAcc,
	r#enum::TaggedEnumValueAcc,
	read::{Read, Ref, SliceRead, StrRead},
};
use crate::{
	de::access::{MapAcc, SeqAcc},
	error::{Error, ErrorCode, Span},
};
use serde::forward_to_deserialize_any;

mod access;
mod r#enum;
mod map;
mod read;

/// a mayfig deseralizer.
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
	/// create a mayfig deserializer from a `&str`.
	#[expect(clippy::should_implement_trait)]
	pub fn from_str(input: &'de str) -> Self {
		let read = StrRead::new(input);
		Deserializer::new(read)
	}
}

/// deserialize a type `T` from a mayfig `&str`
///
/// # errors
///
/// this returns an error if the structure of the input does not match the
/// structure of `T`, or if the `Deserialize` impl of `T` returns an error.
///
/// for more info on possible errors take a look at the
/// [`ErrorCode`] enum
pub fn from_str<'a, T>(input: &'a str) -> Result<T, Error>
where
	T: serde::de::Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_str(input);
	T::deserialize(&mut deserializer)
}

impl<'de> Deserializer<SliceRead<'de>> {
	/// create a mayfig deserializer from a byte slice.
	pub fn from_slice(input: &'de [u8]) -> Self {
		let read = SliceRead::new(input);
		Deserializer::new(read)
	}
}

/// deserialize a type `T` from a mayfig byte slice
///
/// # errors
///
/// this returns an error if the structure of the input does not match the
/// structure of `T`, or if the `Deserialize` impl of `T` returns an error.
///
/// for more info on possible errors take a look at the
/// [`ErrorCode`] enum
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

	fn num<'s>(&'s mut self) -> Result<(Ref<'de, 's, str>, Span), Error> {
		let start = self.read.position();
		self.scratch.clear();

		let r#ref = self.read.num(&mut self.scratch)?;
		if r#ref.is_empty() {
			let peek = self.read.peek().ok_or(Error::EOF)?;

			let point = self.read.position();
			let code = ErrorCode::ExpectedNumeric(peek as char);
			Err(Error::with_point(code, point))
		} else {
			let span = Span::Span(start, self.read.position());
			Ok((r#ref, span))
		}
	}

	fn word<'s>(&'s mut self) -> Result<(Ref<'de, 's, str>, Span), Error> {
		self.scratch.clear();

		let start = self.read.position();
		let word = self.read.word(&mut self.scratch)?;
		Ok((word, Span::Span(start, self.read.position())))
	}

	fn str<'s>(&'s mut self) -> Result<(Ref<'de, 's, str>, Span), Error> {
		let peek = self.read.peek().ok_or(Error::EOF)?;
		if peek != b'"' && peek != b'\'' {
			let point = self.read.position();
			let code = ErrorCode::ExpectedQuote(peek as char);
			return Err(Error::with_point(code, point));
		}

		let start = self.read.position();
		self.scratch.clear();
		let str = self.read.str(&mut self.scratch)?;
		Ok((str, Span::Span(start, self.read.position())))
	}

	fn str_bytes<'s>(&'s mut self) -> Result<Ref<'de, 's, [u8]>, Error> {
		self.scratch.clear();
		self.read.str_bytes(&mut self.scratch)
	}

	fn identifier<'s>(&'s mut self) -> Result<(Ref<'de, 's, str>, Span), Error> {
		let peek = self.read.peek().ok_or(Error::EOF)?;
		if peek == b'"' || peek == b'\'' {
			return self.str();
		} else if !peek.is_ascii_alphabetic() {
			let point = self.read.position();
			let code = ErrorCode::ExpectedAlphabetic(peek as char);
			return Err(Error::with_point(code, point));
		}

		self.word()
	}

	fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value, Error>
	where
		V: serde::de::Visitor<'any>,
	{
		let (w, span) = self.num()?;
		if w.contains('.') || w.contains('e') {
			let n = w.parse::<f64>().map_err(|_| {
				let code = ErrorCode::InvalidNum(w.to_owned());
				Error::with_span(code, span)
			})?;
			visitor.visit_f64(n).map_err(|err| add_span(err, span))
		} else if w.starts_with('-') {
			let n = w.parse::<i64>().map_err(|_| {
				let code = ErrorCode::InvalidNum(w.to_owned());
				Error::with_span(code, span)
			})?;
			visitor.visit_i64(n).map_err(|err| add_span(err, span))
		} else {
			let n = w.parse::<u64>().map_err(|_| {
				let code = ErrorCode::InvalidNum(w.to_owned());
				Error::with_span(code, span)
			})?;
			visitor.visit_u64(n).map_err(|err| add_span(err, span))
		}
	}
}

impl<'de, R: Read<'de>> serde::de::Deserializer<'de> for &mut Deserializer<R> {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let peek = self.peek_any().ok_or(Error::EOF)?;
		if self.indent == 0 || peek == b'{' {
			self.deserialize_map(visitor)
		} else if peek == b'[' {
			self.deserialize_seq(visitor)
		} else if let b'0'..=b'9' | b'.' | b'-' | b'+' = peek {
			self.deserialize_number(visitor)
		} else if peek == b'"' || peek == b'\'' {
			let (str, span) = self.str()?;
			let str = str.to_owned();

			if let Ok(Some(b'[')) = self.peek_line() {
				let tagged = TaggedEnumValueAcc::with_tag(self, str);
				visitor.visit_enum(tagged)
			} else {
				visitor.visit_string(str).map_err(|err| add_span(err, span))
			}
		} else {
			let (word, span) = self.word()?;
			if let Ok(b) = parse_bool(&word, span) {
				visitor.visit_bool(b).map_err(|err| add_span(err, span))
			} else {
				let code = ErrorCode::UnexpectedWord(word.to_owned());
				Err(Error::with_span(code, span))
			}
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.word()?;
		let b = parse_bool(&w, span)?;
		visitor.visit_bool(b).map_err(|err| add_span(err, span))
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<u8>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_u8(n).map_err(|err| add_span(err, span))
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<u16>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_u16(n).map_err(|err| add_span(err, span))
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<u32>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_u32(n).map_err(|err| add_span(err, span))
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<u64>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_u64(n).map_err(|err| add_span(err, span))
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<i8>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_i8(n).map_err(|err| add_span(err, span))
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<i16>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_i16(n).map_err(|err| add_span(err, span))
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<i32>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_i32(n).map_err(|err| add_span(err, span))
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (w, span) = self.num()?;
		let n = w.parse::<i64>().map_err(|_| {
			let code = ErrorCode::InvalidNum(w.to_owned());
			Error::with_span(code, span)
		})?;
		visitor.visit_i64(n).map_err(|err| add_span(err, span))
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
		let (w, span) = self.num()?;
		let n = parse_f64(&w, span)?;
		visitor.visit_f64(n).map_err(|err| add_span(err, span))
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
		let (r#str, span) = self.str()?;
		match r#str {
			Ref::Borrow(b) => visitor.visit_borrowed_str(b).map_err(|e| add_span(e, span)),
			Ref::Scratch(s) => visitor.visit_str(s).map_err(|err| add_span(err, span)),
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
		let peek = self.read.peek().ok_or(Error::EOF)?;
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

		let next = self.read.peek().ok_or(Error::EOF)?;
		if next != b'[' {
			let point = self.read.position();
			let code = ErrorCode::ExpectedSeq(next as char);
			return Err(Error::with_point(code, point));
		}

		let start = self.read.position();
		self.read.discard();

		let seq = SeqAcc::new(self);
		let val = visitor.visit_seq(seq);

		self.discard_commata();
		let next = self.peek_any().ok_or(Error::EOF)?;

		// point for seq end check
		let seq_end = self.read.position();
		self.read.discard();

		let val = val.map_err(|err| {
			let end = self.read.position();
			add_span(err, Span::Span(start, end))
		})?;

		if next != b']' {
			let code = ErrorCode::ExpectedSeqEnd(next as char);
			return Err(Error::with_point(code, seq_end));
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
		let start = self.read.position();

		let peek = self.peek_any();
		let value = if let Some(b'{') = peek {
			self.indent += 1;

			let start = self.read.position();
			self.read.discard();

			let map_acc = MapAcc::new(self);
			let val = visitor.visit_map(map_acc).map_err(|err| {
				let end = self.read.position();
				add_span(err, Span::Span(start, end))
			})?;

			self.indent -= 1;

			Ok(val)
		} else if self.indent != 0 {
			let peek = peek.ok_or(Error::EOF)?;

			let point = self.read.position();
			let code = ErrorCode::ExpectedMap(peek as char);
			return Err(Error::with_point(code, point));
		} else {
			self.indent += 1;

			let top_map_acc = TopMapAcc::new(self);
			let val = visitor.visit_map(top_map_acc).map_err(|err| {
				let end = self.read.position();
				add_span(err, Span::Span(start, end))
			})?;

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
		let peek = self.read.peek().ok_or(Error::EOF)?;
		if peek == b'"' || peek == b'\'' {
			let acc = TaggedEnumValueAcc::new(self);
			visitor.visit_enum(acc)
		} else {
			let point = self.read.position();
			let code = ErrorCode::ExpectedEnum(peek as char);
			Err(Error::with_point(code, point))
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let (identifier, span) = self.identifier()?;
		match identifier {
			Ref::Borrow(b) => visitor.visit_borrowed_str(b).map_err(|e| add_span(e, span)),
			Ref::Scratch(s) => visitor.visit_str(s).map_err(|err| add_span(err, span)),
		}
	}

	forward_to_deserialize_any! { ignored_any }
}

fn parse_bool(word: &str, span: Span) -> Result<bool, Error> {
	if word.eq_ignore_ascii_case("true") {
		Ok(true)
	} else if word.eq_ignore_ascii_case("false") {
		Ok(false)
	} else {
		let code = ErrorCode::InvalidBool(word.to_owned());
		Err(Error::with_span(code, span))
	}
}

fn parse_f64(num: &str, span: Span) -> Result<f64, Error> {
	let stripped = if let Some(stripped) = num.strip_prefix('+') {
		if stripped.starts_with(['+', '-']) {
			let code = ErrorCode::InvalidNum(num.to_owned());
			return Err(Error::with_span(code, span));
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
		Err(Error::with_span(code, span))
	} else if let Ok(float) = stripped.parse::<f64>() {
		if float.is_finite() {
			Ok(float)
		} else {
			unreachable!()
		}
	} else {
		let code = ErrorCode::InvalidNum(num.to_owned());
		Err(Error::with_span(code, span))
	}
}

#[inline]
fn add_span(mut err: Error, span: Span) -> Error {
	err.span = Some(err.span.unwrap_or(span));
	err
}
