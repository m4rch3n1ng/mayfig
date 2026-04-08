use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum Enum {
	Num(usize),
	Vec((usize, usize)),
}

#[derive(Debug, Deserialize)]
#[expect(dead_code)]
struct WithError {
	v: f32,
	m: HashMap<Enum, Enum>,
}

const WITH_ERROR: &str = r#"
ä = ...
m {
	vec [ 0, 1, ] = "vec" . [ 0, 1 ]
	num [ 0 "t" ] = "num" [ 1 ]
}
"#;

fn main() {
	let thing = mayfig::from_str::<WithError>(WITH_ERROR);
	let thing = match thing {
		Ok(thing) => thing,
		Err(err) => {
			let code = err.code().to_string();
			let message = if let Some(span) = err.span() {
				Level::ERROR.secondary_title(code.as_str()).element(
					Snippet::source(WITH_ERROR)
						.path("test/test.mf")
						.fold(true)
						.annotation(AnnotationKind::Primary.span(span.range())),
				)
			} else {
				Group::with_title(Level::ERROR.primary_title(code.as_str()))
			};

			let renderer = Renderer::styled();
			anstream::println!("{}", renderer.render(&[message]));

			return;
		}
	};

	println!("thing {:?}", thing);
}
