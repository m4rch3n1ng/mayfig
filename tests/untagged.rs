use std::io::Cursor;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
struct Top {
	action: Action,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Action {
	Workspace(Workspace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum Workspace {
	Index(usize),
	Motion(WorkspaceMotion),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WorkspaceMotion {
	Next,
	Prev,
}

const T1: &str = r#"
action = "workspace" [ 0 ]
"#;

const T2: &str = r#"
action = "workspace" [ "next" ]
"#;

#[test]
fn test() {
	let t1 @ Top { action: a1 } = mayfig::from_str::<Top>(T1).unwrap();
	let Action::Workspace(w1) = a1;
	assert!(matches!(w1, Workspace::Index(0)));

	let t2 @ Top { action: a2 } = mayfig::from_str::<Top>(T2).unwrap();
	let Action::Workspace(w2) = a2;
	assert!(matches!(w2, Workspace::Motion(WorkspaceMotion::Next)));

	let s1 = mayfig::from_reader::<_, Top>(Cursor::new(T1)).unwrap();
	assert_eq!(s1, t1);

	let s2 = mayfig::from_reader::<_, Top>(Cursor::new(T2)).unwrap();
	assert_eq!(s2, t2);
}
