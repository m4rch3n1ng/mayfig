use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct C<'a> {
	t: u64,
	v: Vec<u64>,
	#[serde(borrow)]
	s: Vec<&'a str>,
}

const C1: &str = r#"
# this is a comment
t = 20# close comment
v = [
	0 1 2 3
] # comment
s = [ # ] comment
]
"#;

const C2: &str = r##"
t = 0 # test
v = [ 0 1 2 ]
s = [
	"test"# another close one
] # test
"##;

#[test]
fn comm() {
	let c1 = mayfig::from_str::<C>(C1);
	let c1 = c1.unwrap();
	assert_eq!(c1.t, 20);
	assert_eq!(c1.v, &[0, 1, 2, 3]);
	assert_eq!(c1.s, Vec::<&str>::new());

	let c2 = mayfig::from_str::<C>(C2);
	let c2 = c2.unwrap();
	assert_eq!(c2.t, 0);
	assert_eq!(c2.v, &[0, 1, 2]);
	assert_eq!(c2.s, &["test"]);
}
