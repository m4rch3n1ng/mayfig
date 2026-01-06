//! when deserializing or serializing mayfig goes wrong

use std::{cmp::Ordering, fmt::Display};

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

	pub(crate) const fn with_point(code: ErrorCode, point: Position) -> Self {
		let span = Some(Span::Point(point));
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

/// a mayfig error code
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorCode {
	/// failed to write into a [`Write`](std::io::Write).
	///
	/// can currently only occur when serializing with [`to_writer`](crate::to_writer).
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

	/// expected delimiter
	ExpectedDelimiter(char),

	/// expected numeric
	ExpectedNumeric(char),
	/// expected alphabetic
	ExpectedAsciiAlphabetic(char),
	/// expected alphanumeric
	ExpectedAsciiAlphanumeric(char),

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
			ErrorCode::ExpectedDelimiter(t) => {
				write!(f, "expected delimiter after string, got {t:?}")
			}
			ErrorCode::ExpectedNumeric(t) => write!(f, "expected ascii numeric, got {t:?}"),
			ErrorCode::ExpectedAsciiAlphabetic(t) => {
				f.write_str("unquoted identifier may only start with ")?;
				write!(f, "ascii letters or _, found {t:?}")
			}
			ErrorCode::ExpectedAsciiAlphanumeric(t) => {
				f.write_str("unquoted identifier may only contain ")?;
				write!(f, "ascii letters, numbers, _, - or +, found {t:?}")
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
	/// 1-indexed
	pub col: usize,
	/// absolute index of the position
	///
	/// 0-indexed
	pub index: usize,
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "line {}, column {}", self.line, self.col)
	}
}

/// span of the `Error` location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Span {
	/// a single point
	Point(Position),
	/// a span between two points
	Span(Position, Position),
}

impl Span {
	/// returns a [`Range`](std::ops::Range) of the span
	pub fn range(&self) -> std::ops::Range<usize> {
		match self {
			Span::Point(pos) => pos.index..pos.index,
			Span::Span(pos1, pos2) => pos1.index..(pos2.index),
		}
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Span::Point(pos) => Display::fmt(pos, f),
			Span::Span(pos1, pos2) => {
				if pos1.line == pos2.line {
					write!(
						f,
						"line {}, columns {} to {}",
						pos1.line, pos1.col, pos2.col
					)
				} else {
					write!(f, "{} to {}", pos1, pos2)
				}
			}
		}
	}
}

impl Ord for Span {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Span::Point(p1), Span::Point(p2)) => p1.cmp(p2),
			(Span::Span(s1_1, s1_2), Span::Span(s2_1, s2_2)) => {
				s1_1.cmp(s2_1).then_with(|| s1_2.cmp(s2_2))
			}
			(Span::Point(p), Span::Span(s1, s2)) => match p.cmp(s1) {
				Ordering::Equal => Ordering::Equal,
				Ordering::Less => Ordering::Less,
				Ordering::Greater => {
					if p > s2 {
						Ordering::Greater
					} else {
						Ordering::Equal
					}
				}
			},
			(Span::Span(s1, s2), Span::Point(p)) => match s1.cmp(p) {
				Ordering::Equal => Ordering::Equal,
				Ordering::Less => {
					if s2 < p {
						Ordering::Less
					} else {
						Ordering::Equal
					}
				}
				Ordering::Greater => Ordering::Greater,
			},
		}
	}
}

impl PartialOrd for Span {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod test {
	use super::{Position, Span};
	use std::cmp::Ordering;

	#[test]
	fn ord_span() {
		let span = Span::Span(
			Position {
				line: 2,
				col: 6,
				index: 15,
			},
			Position {
				line: 2,
				col: 16,
				index: 25,
			},
		);

		let p1 = Span::Point(Position {
			line: 1,
			col: 6,
			index: 5,
		});

		assert_eq!(span.cmp(&p1), Ordering::Greater);
		assert_eq!(p1.cmp(&span), Ordering::Less);

		let p2 = Span::Point(Position {
			line: 2,
			col: 6,
			index: 15,
		});

		assert_eq!(span.cmp(&p2), Ordering::Equal);
		assert_eq!(p2.cmp(&span), Ordering::Equal);

		let p3 = Span::Point(Position {
			line: 2,
			col: 11,
			index: 20,
		});

		assert_eq!(span.cmp(&p3), Ordering::Equal);
		assert_eq!(p3.cmp(&span), Ordering::Equal);

		let p4 = Span::Point(Position {
			line: 2,
			col: 16,
			index: 25,
		});

		assert_eq!(span.cmp(&p4), Ordering::Equal);
		assert_eq!(p4.cmp(&span), Ordering::Equal);

		let p5 = Span::Point(Position {
			line: 3,
			col: 6,
			index: 30,
		});

		assert_eq!(span.cmp(&p5), Ordering::Less);
		assert_eq!(p5.cmp(&span), Ordering::Greater);
	}
}
