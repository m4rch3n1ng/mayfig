use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct T {
	t: Vec<u8>,
}

const T1: &str = r#"
t = [ 2 4 ]
"#;

const T2: &str = r#"
t = [ ,,,, 2 ,, 4 ,,,, ]
"#;

const T3: &str = r#"
t = [
	2
	4
]
"#;

const T4: &str = r#"
t = [
	2,
	4,
]
"#;

#[test]
fn num() {
	let t1 = mayfig::from_str::<T>(T1);
	let t1 = t1.unwrap();
	assert_eq!(&t1.t, &[2, 4]);

	let t2 = mayfig::from_str::<T>(T2);
	let t2 = t2.unwrap();
	assert_eq!(&t2.t, &[2, 4]);

	let t3 = mayfig::from_str::<T>(T3);
	let t3 = t3.unwrap();
	assert_eq!(&t3.t, &[2, 4]);

	let t4 = mayfig::from_str::<T>(T4);
	let t4 = t4.unwrap();
	assert_eq!(&t4.t, &[2, 4]);
}

#[derive(Debug, Deserialize)]
struct V<'a> {
	#[serde(borrow)]
	v: (&'a str, u32, &'a str),
}

const V1: &str = r#"
v = [ "test" 2 "three" ]
"#;

const V2: &str = r#"
v = [
	"test",
	2,
	"three",
]
"#;

#[test]
fn string() {
	let v1 = mayfig::from_str::<V>(V1);
	let v1 = v1.unwrap();
	assert_eq!(v1.v, ("test", 2, "three"));

	let v2 = mayfig::from_str::<V>(V2);
	let v2 = v2.unwrap();
	assert_eq!(v2.v, ("test", 2, "three"));
}
