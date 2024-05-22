pub mod de;
pub mod error;

#[doc(inline)]
pub use de::{from_slice, from_str, Deserializer};
#[doc(inline)]
pub use error::Error;
