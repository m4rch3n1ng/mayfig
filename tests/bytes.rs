use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct Byt<'a> {
	#[serde(with = "serde_bytes")]
	#[serde(borrow)]
	s: Cow<'a, [u8]>,
	#[serde(with = "serde_bytes")]
	v: Vec<u8>,
	e: E<'a>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum E<'a> {
	#[serde(borrow, with = "serde_bytes")]
	Byt(Cow<'a, [u8]>),
}

const B1: &str = r#"
s = "test"
v = [ 116 101 115 116 ]
e = "byt" [ "val" ]
"#;

const B2: &str = r#"
s = [ 195 164 229 173 151 ]
v = "ä字"
e = "byt" [ 0 1 3 4 ]
"#;

#[test]
fn bytes() {
	let b1 = mayfig::from_str::<Byt>(B1);
	let b1 = b1.unwrap();
	assert_eq!(b1.s, "test".as_bytes());
	assert_eq!(b1.s, b1.v);
	assert_eq!(b1.e, E::Byt(Cow::Borrowed("val".as_bytes())));
	assert!(matches!(b1.s, Cow::Borrowed(_)));

	let b2 = mayfig::from_str::<Byt>(B2);
	let b2 = b2.unwrap();
	assert_eq!(b2.s, "ä字".as_bytes());
	assert_eq!(b2.s, b2.v);
	assert_eq!(b2.e, E::Byt(Cow::Owned(vec![0, 1, 3, 4])));
	assert!(matches!(b2.s, Cow::Owned(_)));
}

#[derive(Debug, Deserialize)]
struct Wtf<'a> {
	#[serde(with = "serde_bytes")]
	#[serde(borrow)]
	uh: Cow<'a, [u8]>,
}

const WTF: &[u8] = &[
	b'u', b'h', b' ', b'=', b' ', b'"', 255, 255, 128, 255, b'"', b'\n',
];

#[test]
#[expect(invalid_from_utf8)]
fn fucked() {
	assert!(std::str::from_utf8(WTF).is_err());

	let wtf = mayfig::from_slice::<Wtf>(WTF);
	let wtf = wtf.unwrap();
	assert_eq!(&*wtf.uh, &[255, 255, 128, 255]);
	assert!(matches!(wtf.uh, Cow::Borrowed(_)));
}
