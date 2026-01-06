use crate::{de::read::Read, Deserializer, Error};
use serde_core::de::{EnumAccess, VariantAccess};

pub struct TaggedUnitEnumAcc<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read<'de>> TaggedUnitEnumAcc<'a, R> {
	pub fn new(de: &'a mut Deserializer<R>) -> Self {
		TaggedUnitEnumAcc { de }
	}
}

impl<'de, R: Read<'de>> EnumAccess<'de> for TaggedUnitEnumAcc<'_, R> {
	type Error = Error;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde_core::de::DeserializeSeed<'de>,
	{
		let variant = seed.deserialize(&mut *self.de)?;
		Ok((variant, self))
	}
}

impl<'de, R: Read<'de>> VariantAccess<'de> for TaggedUnitEnumAcc<'_, R> {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	#[expect(unused_variables)]
	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde_core::de::DeserializeSeed<'de>,
	{
		todo!("nested tagged values");
	}

	#[expect(unused_variables)]
	fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		todo!("nested tagged values");
	}

	#[expect(unused_variables)]
	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		todo!("nested tagged values");
	}
}
