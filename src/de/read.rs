pub trait Read<'de> {}

pub struct StrRead<'de> {
	data: std::marker::PhantomData<&'de ()>,
}

impl<'de> StrRead<'de> {
	pub fn new(_input: &str) -> Self {
		todo!()
	}
}

impl<'de> Read<'de> for StrRead<'de> {}
