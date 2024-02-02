use super::*;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;

pub trait NumberFieldValue:
	'static + FieldValue + PartialOrd + Display + FromStr
{
}
impl<T: 'static + FieldValue + PartialOrd + Display + FromStr> NumberFieldValue
	for T
{
}


pub struct NumberField<T: NumberFieldValue> {
	pub reflect: FieldReflect<T>,
	pub step: T,
}

impl<T: NumberFieldValue> NumberField<T> {
	pub fn new(
		field_name: String,
		get_cb: impl 'static + Fn() -> T,
		set_cb: impl 'static + Fn(T),
		step: T,
	) -> Self {
		Self {
			reflect: FieldReflect::new(field_name, get_cb, set_cb),
			step,
		}
	}
	pub fn from_reflect(reflect: FieldReflect<T>, step: T) -> Self {
		Self { reflect, step }
	}
}

impl<T: NumberFieldValue> Deref for NumberField<T> {
	type Target = FieldReflect<T>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}

impl<T: NumberFieldValue> DerefMut for NumberField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: NumberFieldValue> Display for NumberField<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NumberField")
			.field("name", &self.reflect.field_name)
			.field("value", &self.reflect.get().to_string())
			.field("step", &self.step.to_string())
			.finish()
	}
}

impl IntoFieldUi for u8 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberU8(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for u16 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberU16(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for u32 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberU32(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for u64 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberU64(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for i8 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberI8(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for i16 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberI16(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for i32 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberI32(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for i64 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberI64(NumberField { reflect, step: 1 })
	}
}

impl IntoFieldUi for f32 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberF32(NumberField { reflect, step: 1.0 })
	}
}

impl IntoFieldUi for f64 {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		FieldUi::NumberF64(NumberField { reflect, step: 1.0 })
	}
}
