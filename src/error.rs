use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("end of file")]
	Eof,
	#[error("invalid utf8")]
	InvalidUtf8,

	#[error("unknown escpae sequence \"\\{0}\"")]
	UnknownEscape(char),
	#[error("unescaped control character {0:?}")]
	UnescapedControl(char),

	#[error("expected newline")]
	ExpectedNewline,
	#[error("unexpected newline")]
	UnexpectedNewline,

	#[error("invalid number {0:?}")]
	InvalidNum(String),

	#[error("expected quote \" or ', got {0:?}")]
	ExpectedQuote(char),
	#[error("expected value assignment '=', '{{', got {0:?}")]
	ExpectedValue(char),
	#[error("expected map '{{', got {0:?}")]
	ExpectedMap(char),
	#[error("expected end of map '}}', got {0:?}")]
	ExpectedMapEnd(char),
	#[error("expected seq '[', got {0:?}")]
	ExpectedSeq(char),
	#[error("expected end of seq ']', got {0:?}")]
	ExpectedSeqEnd(char),

	#[error("expected delimiter after string, got {0:?}")]
	ExpectedDelimiter(char),

	#[error("expected numeric, got {0:?}")]
	ExpectedNumeric(char),
	#[error("expected alphabetic, got {0:?}")]
	ExpectedAlphabetic(char),
	#[error("expected alphanumeric, got {0:?}")]
	ExpectedAlphaNumeric(char),

	#[error("unsupported unit type")]
	UnsupportedUnit,

	#[error("custom: {0}")]
	Custom(String),
}

impl serde::de::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		let msg = msg.to_string();
		Error::Custom(msg)
	}
}
