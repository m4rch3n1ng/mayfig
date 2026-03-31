//! # mayfig
//!
//! `mayfig` is a config format designed for the
//! [`mayland`](https://github.com/m4rch3n1ng/mayland) wayland compositor, though
//! it can also be used on its own.
//!
//! mayfig is, at its core, a `key=value` config format and is
//! supposed to look similar-ish to toml, hyprlang and the sway config format.
//!
//! ```text
//! input {
//!     keyboard {
//!         # xkb-file = "~/.config/keymap/may.xkb"
//!
//!         repeat-delay = 600
//!         repeat-rate = 25
//!     }
//!
//!     touchpad {
//!         tap = true
//!
//!         natural-scroll = true
//!         # scroll-method = "two-finger"
//!     }
//! }
//!
//! cursor {
//!     xcursor-theme = "Bibata-Modern-Classic"
//!     xcursor-size = 24
//! }
//!
//! bind {
//!     mod+escape = "quit"
//!     mod+q = "close"
//!
//!     mod+t = "spawn" [ "kitty" ]
//!     mod+n = "spawn" [ "firefox" ]
//! }
//! ```
//!
//! for a more thorough explanation take a look at the readme on
//! [`github`](https://github.com/m4rch3n1ng/mayfig) or on crates.io.
//!
//! ## Deserialization and Serialization
//!
//! to deserialize or serialize a type, that type will have to implement the
//! [`serde`](https://serde.rs) traits `Deserialize` or `Serialize`.
//!
//! an example of deserializing with mayfig may look like this:
//!
//! ```
//! use serde::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     pub theme: String,
//!     pub autosave: bool,
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = r#"
//! theme = "solarized-light"
//! autosave = true
//! "#;
//!
//! let conf = mayfig::from_str::<Config>(config)?;
//! println!("{:?}", conf);
//!
//! # Ok(())
//! # }
//! ```
//!
//! and serialization may look like this:
//!
//! ```
//! use serde::Serialize;
//!
//! #[derive(Debug, Serialize)]
//! struct Config {
//!     pub theme: String,
//!     pub autosave: bool,
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let conf = Config {
//!     theme: "solarized-light".to_owned(),
//!     autosave: true,
//! };
//!
//! let string = mayfig::to_string(&conf)?;
//! println!("{}", string);
//!
//! # Ok(())
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

pub mod de;
pub mod error;
pub mod ser;
#[cfg(feature = "value")]
pub mod value;

#[doc(inline)]
pub use de::{from_str, Deserializer};
#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use ser::{to_string, to_vec, to_writer, Serializer};
#[doc(inline)]
#[cfg(feature = "value")]
pub use value::Value;
