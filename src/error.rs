//! when deserializing or serializing mayfig goes wrong

use std::{cmp::Ordering, fmt::Display};
use thiserror::Error;

/// a `mayfig::Error`
#[derive(Debug)]
pub struct Error {
	code: ErrorCode,
	span: Option<Span>,
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

impl serde::de::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: Display,
	{
		let msg = msg.to_string();
		let code = ErrorCode::Custom(msg);
		Error::new(code)
	}
}

impl serde::ser::Error for Error {
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
#[derive(Debug, Error)]
pub enum ErrorCode {
	/// failed to write into a [`Write`](std::io::Write).
	///
	/// can currently only occur when serializing with [`to_writer`](crate::to_writer).
	#[error("io error")]
	Io(#[source] std::io::Error),

	/// unexpected end of file while parsing
	#[error("end of file")]
	Eof,
	/// mayfig value is invalid utf8
	#[error("invalid utf8")]
	InvalidUtf8,

	/// unknown escape sequence in string
	#[error("unknown escape sequence {0:?}")]
	UnknownEscape(char),
	/// unescaped control character in string
	#[error("unescaped control character {0:?}")]
	UnescapedControl(char),

	/// expected newline, found value
	#[error("expected newline, found {0:?} first")]
	ExpectedNewline(char),
	/// expected value, found newline
	#[error("unexpected newline")]
	UnexpectedNewline,

	/// invalid boolean
	#[error("invalid boolean {0:?}")]
	InvalidBool(String),
	/// invalid number
	#[error("invalid number {0:?}")]
	InvalidNum(String),
	/// unexpected word
	#[error("unexpected word {0:?}")]
	UnexpectedWord(String),

	/// expected quote
	#[error("expected quote \" or ', got {0:?}")]
	ExpectedQuote(char),
	/// expected value assignment
	#[error("expected value assignment '=' or '{{', got {0:?}")]
	ExpectedValue(char),
	/// expected map
	#[error("expected map '{{', got {0:?}")]
	ExpectedMap(char),
	/// expected sequence
	#[error("expected seq '[', got {0:?}")]
	ExpectedSeq(char),
	/// expected end of sequence
	#[error("expected end of seq ']', got {0:?}")]
	ExpectedSeqEnd(char),
	/// expected enum
	#[error("expected tagged enum, got {0:?}")]
	ExpectedEnum(char),
	/// expected bytes as string or sequence
	#[error("expected quote ', \" or seq, got {0:?}")]
	ExpectedBytes(char),

	/// expected delimiter
	#[error("expected delimiter after string, got {0:?}")]
	ExpectedDelimiter(char),

	/// expected numeric
	#[error("expected numeric, got {0:?}")]
	ExpectedNumeric(char),
	/// expected alphabetic
	#[error("expected alphabetic, got {0:?}")]
	ExpectedAlphabetic(char),
	/// expected alphanumeric
	#[error("expected alphanumeric, got {0:?}")]
	ExpectedAlphaNumeric(char),

	/// unit values are unsupported in mayfig
	#[error("unsupported unit type")]
	UnsupportedUnit,
	/// nan is unsupported in mayfig
	#[error("unsupported nan")]
	UnsupportedNaN,
	/// `None` is unsupported in mayfig
	#[error("unsupported none")]
	UnsupportedNone,
	/// unsupported map key
	#[error("unsupported map key type {0}")]
	UnsupportedMapKey(&'static str),

	/// custom serde error
	#[error("custom: {0}")]
	Custom(String),
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
			Span::Point(pos) => pos.index..(pos.index + 1),
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
