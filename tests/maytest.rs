use mayfig::error::ErrorCode;

#[expect(clippy::allow_attributes)]
#[allow(dead_code, reason = "test weirdness")]
pub fn errorcode_to_str(code: &ErrorCode) -> Option<String> {
	match code {
		ErrorCode::UnknownEscape(ch)
		| ErrorCode::UnescapedControl(ch)
		| ErrorCode::ExpectedNewline(ch)
		| ErrorCode::ExpectedQuote(ch)
		| ErrorCode::ExpectedValue(ch)
		| ErrorCode::ExpectedMap(ch)
		| ErrorCode::ExpectedSeq(ch)
		| ErrorCode::ExpectedSeqEnd(ch)
		| ErrorCode::ExpectedEnum(ch)
		| ErrorCode::ExpectedBytes(ch)
		| ErrorCode::ExpectedRegex(ch)
		| ErrorCode::ExpectedDelimiter(ch)
		| ErrorCode::ExpectedNumeric(ch)
		| ErrorCode::ExpectedWordStart(ch)
		| ErrorCode::ExpectedWordContinue(ch)
		| ErrorCode::ExpectedRegexFlag(ch) => Some(ch.to_string()),

		ErrorCode::InvalidBool(s) | ErrorCode::InvalidNum(s) | ErrorCode::UnexpectedWord(s) => {
			Some(s.to_owned())
		}

		ErrorCode::Io(_)
		| ErrorCode::Eof
		| ErrorCode::InvalidUtf8
		| ErrorCode::UnexpectedNewline
		| ErrorCode::UnsupportedUnit
		| ErrorCode::UnsupportedNaN
		| ErrorCode::UnsupportedNone
		| ErrorCode::UnsupportedMapKey(_)
		| ErrorCode::Custom(_) => None,

		_ => unreachable!("missed match arm for {code:?}"),
	}
}

#[macro_export]
macro_rules! assert_de {
	($string:ident as $ty:ty, $val:expr) => {{
		let sde = mayfig::from_str::<$ty>($string).unwrap();
		assert_eq!(sde, $val);

		let rde = mayfig::from_reader::<_, $ty>(std::io::Cursor::new($string)).unwrap();
		assert_eq!(rde, $val);

		sde
	}};

	($string:ident as $ty:ty => $name:ident, $act:expr, $val:expr) => {{
		let $name = mayfig::from_str::<$ty>($string).unwrap();
		assert_eq!($act, $val);

		let $name = mayfig::from_reader::<_, $ty>(std::io::Cursor::new($string)).unwrap();
		assert_eq!($act, $val);

		$name
	}};
}

#[macro_export]
macro_rules! assert_err {
	($string:ident as $ty: ty, $code:pat) => {{
		let se = mayfig::from_str::<$ty>($string).unwrap_err();
		assert!(matches!(se.code(), $code));

		let re = mayfig::from_reader::<_, $ty>(std::io::Cursor::new($string)).unwrap_err();
		assert!(matches!(re.code(), $code));

		assert_eq!(se.span(), re.span());
		if let Some(span) = se.span() {
			let slice = &$string[span.range()];
			assert!(!slice.is_empty(), "error span should not be empty");

			if let Some(code) = maytest::errorcode_to_str(se.code()) {
				assert_eq!(code, slice);
			}
		}
	}};

	($string:ident as $ty: ty, $code:pat, $span:expr) => {{
		let se = mayfig::from_str::<$ty>($string).unwrap_err();
		assert!(matches!(se.code(), $code));
		assert_eq!(se.span(), Some($span));

		let re = mayfig::from_reader::<_, $ty>(std::io::Cursor::new($string)).unwrap_err();
		assert!(matches!(re.code(), $code));
		assert_eq!(re.span(), Some($span));

		let slice = &$string[$span.range()];
		assert!(!slice.is_empty(), "error span should not be empty");

		if let Some(code) = maytest::errorcode_to_str(se.code()) {
			assert_eq!(code, slice);
		}
	}};
}
