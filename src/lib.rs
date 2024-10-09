//! # mayfig
//!
//! `mayfig` is a config format designed for the [`mayland`](https://github.com/m4rch3n1ng/mayland) compositor.
//!
//! mayfig is, at its core, a `key=value` config format and is
//! supposed to look similar-ish to toml, hyprlang and the sway config format.
//!
//! ```text
//! input {
//!     keyboard {
//!         # xkb_file = "~/.config/keymap/may.xkb"
//!
//!         repeat_delay = 600
//!         repeat_rate = 25
//!     }
//!
//!     touchpad {
//!         tap = true
//!
//!         natural_scroll = true
//!         scroll_method = "two_finger"
//!     }
//! }
//!
//! cursor {
//!     xcursor_theme = "Bibata-Modern-Classic"
//!     xcursor_size = 24
//! }
//!
//! bind {
//!     "mod escape" = "quit"
//!     "mod q" = "close"
//!
//!     "mod t" = "spawn" [ "kitty" ]
//!     "mod n" = "spawn" [ "firefox" ]
//! }
//! ```
//!
//! for a more thorough explanation take a look at the readme on
//! [`github`](https://github.com/m4rch3n1ng/mayfig) or on crates.io.

pub mod de;
pub mod error;
pub mod ser;
pub mod value;

#[doc(inline)]
pub use de::{from_slice, from_str, Deserializer};
#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use ser::{to_string, to_vec, to_writer, Serializer};
#[doc(inline)]
pub use value::Value;
