use serde::Deserialize;

#[derive(Deserialize)]
struct Top {
	action: Action,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Action {
	Workspace(Workspace),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Workspace {
	Index(usize),
	Motion(WorkspaceMotion),
}

#[derive(Deserialize)]
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
	let Top { action: a1 } = mayfig::from_str::<Top>(T1).unwrap();
	let Action::Workspace(t1) = a1;
	assert!(matches!(t1, Workspace::Index(0)));

	let Top { action: a2 } = mayfig::from_str::<Top>(T2).unwrap();
	let Action::Workspace(t2) = a2;
	assert!(matches!(t2, Workspace::Motion(WorkspaceMotion::Next)));
}
