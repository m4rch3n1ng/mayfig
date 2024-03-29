use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct UnitS {
	u: Unit,
}

#[derive(Debug, Deserialize, PartialEq)]
enum Unit {
	One,
	Two,
}

const U1: &str = r#"
u = "One"
"#;

const U2: &str = r#"
u = "Two"
"#;

const U3: &str = r#"
u = "two"
"#;

const U4: &str = r#"
u {
	Two = []
}
"#;

#[test]
fn unit() {
	let u1 = mayfig::from_str::<UnitS>(U1);
	let u1 = u1.unwrap();
	assert_eq!(u1.u, Unit::One);

	let u2 = mayfig::from_str::<UnitS>(U2);
	let u2 = u2.unwrap();
	assert_eq!(u2.u, Unit::Two);

	let u3 = mayfig::from_str::<UnitS>(U3);
	assert!(u3.is_err());

	let u4 = mayfig::from_str::<UnitS>(U4);
	assert!(u4.is_err());
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "t", content = "c")]
enum Adj {
	Res(String),
	Err { code: u32, msg: String },
}

#[derive(Debug, Deserialize)]
struct Ad {
	adj: Adj,
}

const AD: &str = r#"
adj {
	t = "Res"
	c = "code"
}
"#;

const AD2: &str = r#"
adj {
	t = "Err"
	c {
		code = 404
		msg = "not found"
	}
}
"#;

#[test]
fn adj() {
	let ad1 = mayfig::from_str::<Ad>(AD);
	let ad1 = ad1.unwrap();
	assert_eq!(ad1.adj, Adj::Res("code".into()));

	let ad2 = mayfig::from_str::<Ad>(AD2);
	let ad2 = ad2.unwrap();
	assert_eq!(
		ad2.adj,
		Adj::Err {
			code: 404,
			msg: "not found".into()
		}
	)
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
enum Internal {
	Res { code: u32, msg: String },
	Err { code: u32, msg: String },
}

#[derive(Debug, Deserialize)]
struct In {
	int: Internal,
}

const IN1: &str = r#"
int {
	type = "Res"
	code = 200
	msg = "ok"
}
"#;

#[test]
fn internal() {
	let in1 = mayfig::from_str::<In>(IN1);
	let in1 = in1.unwrap();
	assert_eq!(
		in1.int,
		Internal::Res {
			code: 200,
			msg: "ok".into()
		}
	)
}

#[derive(Debug, Deserialize)]
struct Norm<'a> {
	#[serde(borrow)]
	n: Normal<'a>,
}

#[derive(Debug, Deserialize)]
enum Normal<'a> {
	Id(&'a str),
	St { code: u64 },
	Un,
}

const N1: &str = r#"
n {
	Id = "one"
}
"#;

const N2: &str = r#"
n {
	St {
		code = 200
	}
}
"#;

const N3: &str = r#"
n = "Un"
"#;

const N4: &str = r#"
n = "St"
"#;

const N5: &str = r#"
n {
	Id {
		code = 200
	}
}
"#;

#[test]
fn normal() {
	let n1 = mayfig::from_str::<Norm>(N1);
	let n1 = n1.unwrap();
	assert!(matches!(n1.n, Normal::Id("one")));

	let n2 = mayfig::from_str::<Norm>(N2);
	let n2 = n2.unwrap();
	assert!(matches!(n2.n, Normal::St { code: 200 }));

	let n3 = mayfig::from_str::<Norm>(N3);
	let n3 = n3.unwrap();
	assert!(matches!(n3.n, Normal::Un));

	let n4 = mayfig::from_str::<Norm>(N4);
	assert!(n4.is_err());

	let n5 = mayfig::from_str::<Norm>(N5);
	assert!(n5.is_err());
}
