use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct N {
	n: NT,
}

#[derive(Debug, Deserialize)]
struct NT(u32);

const N1: &str = r#"
n = 4
"#;

#[test]
fn newtype() {
	let n1 = mayfig::from_str::<N>(N1).unwrap();
	assert_eq!(n1.n.0, 4);
}

#[derive(Debug, Deserialize)]
struct O {
	o: Option<u64>,
}

const O1: &str = r#""#;

const O2: &str = r#"
o = 4
"#;

#[test]
fn option() {
	let o1 = mayfig::from_str::<O>(O1);
	let o1 = o1.unwrap();
	assert_eq!(o1.o, None);

	let o2 = mayfig::from_str::<O>(O2);
	let o2 = o2.unwrap();
	assert_eq!(o2.o, Some(4));
}

#[derive(Debug, Deserialize)]
struct T {
	t: (u64, u64, u64),
}

const T1: &str = r#"
t [ 1 2 3 ]
"#;

const T2: &str = r#"
t [ 1 2 ]
"#;

const T3: &str = r#"
t [ 1 2 3 4 ]
"#;

#[derive(Debug, Deserialize)]
struct TN {
	t: (u64, String, bool),
}

const T4: &str = r#"
t [ 2 "test" yes ]
"#;

#[derive(Debug, Deserialize)]
struct TT {
	t: TS,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TS(f64, bool);

const T5: &str = r#"
t [ 2.4, no ]
"#;

#[test]
fn tuple() {
	let t1 = mayfig::from_str::<T>(T1);
	let t1 = t1.unwrap();
	assert_eq!(t1.t, (1, 2, 3));

	let t2 = mayfig::from_str::<T>(T2);
	assert!(t2.is_err());

	let t3 = mayfig::from_str::<T>(T3);
	assert!(t3.is_err());

	let t4 = mayfig::from_str::<TN>(T4);
	let t4 = t4.unwrap();
	assert_eq!(t4.t, (2, "test".into(), true));

	let t5 = mayfig::from_str::<TT>(T5);
	let t5 = t5.unwrap();
	assert_eq!(t5.t, TS(2.4, false))
}
