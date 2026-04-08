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
	}};
}
