//! when deserializing or serializing mayfig goes wrong

use std::fmt::Display;

/// a `mayfig::Error`
#[derive(Debug)]
pub struct Error {
	pub(crate) code: ErrorCode,
	pub(crate) span: Option<Span>,
}

impl Error {
	/// returns a reference to the underlying [`ErrorCode`]
	pub fn code(&self) -> &ErrorCode {
		&self.code
	}

	/// returns an optional span of the location where the error happened
	///
	/// only ever returns `Some` when deserializing.
	pub fn span(&self) -> Option<Span> {
		self.span
	}
}

impl Error {
	pub(crate) const EOF: Error = Error::new(ErrorCode::Eof);

	pub(crate) const fn new(code: ErrorCode) -> Self {
		Error { code, span: None }
	}

	pub(crate) const fn with_point(code: ErrorCode, start: Position, ch: char) -> Self {
		let mut end = start;
		end.next(ch);

		let span = Some(Span::new(start, end));
		Error { code, span }
	}

	pub(crate) const fn with_span(code: ErrorCode, span: Span) -> Self {
		Error {
			code,
			span: Some(span),
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(span) = self.span {
			write!(f, "{} at {}", self.code, span)
		} else {
			Display::fmt(&self.code, f)
		}
	}
}

impl std::error::Error for Error {}

impl serde_core::de::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: Display,
	{
		let msg = msg.to_string();
		let code = ErrorCode::Custom(msg);
		Error::new(code)
	}
}

impl serde_core::ser::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: Display,
	{
		let msg = msg.to_string();
		let code = ErrorCode::Custom(msg);
		Error::new(code)
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		let code = ErrorCode::Io(err);
		Error::new(code)
	}
}

impl From<utf8_decode::Utf8Error> for Error {
	fn from(_value: utf8_decode::Utf8Error) -> Self {
		let code = ErrorCode::InvalidUtf8;
		Error::new(code)
	}
}

/// a mayfig error code
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorCode {
	/// failed to write into a [`Write`](std::io::Write).
	///
	/// can only occur when deserializing with [`from_reader`](crate::from_reader)
	/// or serializing with [`to_writer`](crate::to_writer)
	Io(std::io::Error),

	/// unexpected end of file while parsing
	Eof,
	/// mayfig value is invalid utf8
	InvalidUtf8,

	/// unknown escape sequence in string
	UnknownEscape(char),
	/// unescaped control character in string
	UnescapedControl(char),

	/// expected newline, found value
	ExpectedNewline(char),
	/// expected value, found newline
	UnexpectedNewline,

	/// invalid boolean
	InvalidBool(String),
	/// invalid number
	InvalidNum(String),
	/// unexpected word
	UnexpectedWord(String),

	/// expected quote
	ExpectedQuote(char),
	/// expected value assignment
	ExpectedValue(char),
	/// expected map
	ExpectedMap(char),
	/// expected sequence
	ExpectedSeq(char),
	/// expected end of sequence
	ExpectedSeqEnd(char),
	/// expected enum
	ExpectedEnum(char),
	/// expected bytes as string or sequence
	ExpectedBytes(char),
	/// expected regex
	ExpectedRegex(char),

	/// expected delimiter
	ExpectedDelimiter(char),

	/// expected numeric
	ExpectedNumeric(char),
	/// expected word start character
	ExpectedWordStart(char),
	/// expected word continue character
	ExpectedWordContinue(char),

	/// unit values are unsupported in mayfig
	UnsupportedUnit,
	/// nan is unsupported in mayfig
	UnsupportedNaN,
	/// `None` is unsupported in mayfig
	UnsupportedNone,
	/// unsupported map key
	UnsupportedMapKey(&'static str),

	/// custom serde error
	Custom(String),
}

impl std::error::Error for ErrorCode {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			ErrorCode::Io(source) => Some(source),
			_ => None,
		}
	}
}

impl Display for ErrorCode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ErrorCode::Io(_) => f.write_str("io error"),
			ErrorCode::Eof => f.write_str("end of file"),
			ErrorCode::InvalidUtf8 => f.write_str("invalid utf8"),
			ErrorCode::UnknownEscape(t) => write!(f, "unknown escape sequence {t:?}"),
			ErrorCode::UnescapedControl(ch) => write!(f, "unescaped control character {ch:?}"),
			ErrorCode::ExpectedNewline(t) => write!(f, "expected newline, found {t:?} first"),
			ErrorCode::UnexpectedNewline => f.write_str("unexpected newline"),
			ErrorCode::InvalidBool(t) => write!(f, "invalid boolean {t:?}"),
			ErrorCode::InvalidNum(t) => write!(f, "invalid number {t:?}"),
			ErrorCode::UnexpectedWord(t) => write!(f, "unexpected word {t:?}"),
			ErrorCode::ExpectedQuote(t) => write!(f, "expected quote \" or ', got {t:?}"),
			ErrorCode::ExpectedValue(t) => {
				write!(f, "expected value assignment '=' or '{{', got {t:?}")
			}
			ErrorCode::ExpectedMap(t) => write!(f, "expected map '{{', got {t:?}"),
			ErrorCode::ExpectedSeq(t) => write!(f, "expected seq '[', got {t:?}"),
			ErrorCode::ExpectedSeqEnd(t) => write!(f, "expected end of seq ']', got {t:?}"),
			ErrorCode::ExpectedEnum(t) => write!(f, "expected tagged enum, got {t:?}"),
			ErrorCode::ExpectedBytes(t) => write!(f, "expected quote ', \" or seq, got {t:?}"),
			ErrorCode::ExpectedRegex(t) => write!(f, "expected regex '/', got {t:?}"),
			ErrorCode::ExpectedDelimiter(t) => {
				write!(f, "expected delimiter after string, got {t:?}")
			}
			ErrorCode::ExpectedNumeric(t) => write!(f, "expected ascii numeric, got {t:?}"),
			ErrorCode::ExpectedWordStart(t) => {
				f.write_str("unquoted identifier may only start with ")?;
				write!(f, "ascii letters, _ or *, found {t:?}")
			}
			ErrorCode::ExpectedWordContinue(t) => {
				f.write_str("unquoted identifier may only contain ")?;
				write!(f, "ascii letters, numbers, _, *, - or +, found {t:?}")
			}
			ErrorCode::UnsupportedUnit => f.write_str("unsupported unit type"),
			ErrorCode::UnsupportedNaN => f.write_str("unsupported nan"),
			ErrorCode::UnsupportedNone => f.write_str("unsupported none"),
			ErrorCode::UnsupportedMapKey(t) => write!(f, "unsupported map key type {t}"),
			ErrorCode::Custom(t) => write!(f, "{t}"),
		}
	}
}

/// an error position
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	/// line at which the error occured
	///
	/// 1-indexed
	pub line: usize,
	/// column at which the error occured
	///
	/// 1-indexed, in chars
	pub col: usize,
	/// absolute index of the position
	///
	/// 0-indexed, in bytes
	pub index: usize,
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "line {}, column {}", self.line, self.col)
	}
}

/// span of the `Error` location
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
	/// the start position of the span
	pub start: Position,
	/// the end position of the span, exclusive
	pub end: Position,
}

impl Span {
	/// create a new Span
	pub const fn new(start: Position, end: Position) -> Self {
		Span { start, end }
	}

	/// returns a [`Range`](std::ops::Range) of the span
	pub fn range(&self) -> std::ops::Range<usize> {
		self.start.index..self.end.index
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Span { start, end } = &self;
		if start.line == end.line {
			write!(
				f,
				"line {}, columns {} to {}",
				start.line, start.col, end.col
			)
		} else {
			write!(f, "{} to {}", start, end)
		}
	}
}
