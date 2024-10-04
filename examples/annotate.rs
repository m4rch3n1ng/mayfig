use annotate_snippets::{Level, Renderer, Snippet};
use serde_derive::Deserialize;
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
v = ...
m {
	vec [ 0, 1, ] = "vec" . [ 0, 1 ]
	num [ 0, ] = "num" [ 1 ]
}
"#;

fn main() {
	// blocked on https://github.com/rust-lang/annotate-snippets-rs/issues/25
	let with_error = WITH_ERROR.replace('\t', "    ");

	let thing = mayfig::from_str::<WithError>(&with_error);
	let thing = match thing {
		Ok(thing) => thing,
		Err(err) => {
			let code = err.code().to_string();
			let message = if let Some(span) = err.span() {
				Level::Error.title(code.as_str()).snippet(
					Snippet::source(&with_error)
						.origin("test/test.mf")
						.fold(true)
						.annotation(Level::Error.span(span.range())),
				)
			} else {
				Level::Error.title(code.as_str())
			};

			let renderer = Renderer::styled();
			anstream::println!("{}", renderer.render(message));

			return;
		}
	};

	println!("thing {:?}", thing);
}
