use serde::Deserialize;

mod maytest;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct C {
	t: u64,
	v: Vec<u64>,
	s: Vec<String>,
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
	assert_de!(
		C1 as C,
		C {
			t: 20,
			v: vec![0, 1, 2, 3],
			s: vec![]
		}
	);
	assert_de!(
		C2 as C,
		C {
			t: 0,
			v: vec![0, 1, 2],
			s: vec!["test".to_owned()]
		}
	);
}
