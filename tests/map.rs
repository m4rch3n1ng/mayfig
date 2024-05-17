use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct T {
	n: N,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct N {
	t: u32,
}

const T1: &str = r#"
n {
	t = 20
}
"#;

const T2: &str = r#"{
	n { t = 20
	}
}"#;

const T3: &str = r#"
n = {
	t = 20
}
"#;

#[test]
fn map() {
	let t1 = mayfig::from_str::<T>(T1);
	let t1 = t1.unwrap();
	assert_eq!(t1.n, N { t: 20 });

	let t2 = mayfig::from_str::<T>(T2);
	let t2 = t2.unwrap();
	assert_eq!(t2.n, N { t: 20 });

	let t3 = mayfig::from_str::<T>(T3);
	let t3 = t3.unwrap();
	assert_eq!(t3.n, N { t: 20 });
}
