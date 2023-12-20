// use std::ops::Deref;
// use std::ops::DerefMut;

// /// Trait for structs that implement Deref and DerefMut, required for use as a prop.
// pub trait Prop<T>: Deref<Target = T> + DerefMut<Target = T> {}
// impl<T> Prop<T> for T where T: Deref<Target = T> + DerefMut<Target = T> {}
// /// Trait for structs that implement Deref and DerefMut, required for use as a prop.
// pub trait Prop<T> {
// 	fn get(&self) -> &T;
// 	fn set(&mut self, value: T);
// }
// impl<T> Prop<T> for T where T: Deref<Target = T> + DerefMut<Target = T> {}


// pub trait PropList<T> {
// 	fn get_props() -> Vec<Box<dyn Prop<T>>>;
// }

pub struct SetPropError(pub String);

pub struct PropHandle<PropT, ValueT> {
	pub get: fn(prop: &PropT) -> ValueT,
	pub set: fn(prop: &mut PropT, value: ValueT) -> Result<(), SetPropError>,
	pub show_if: fn(prop: &PropT) -> bool,
}

pub trait PropInput<T> {
	fn get(&self) -> &T;
	fn set(&mut self, value: T);
	fn show_if(&self) -> bool { true }
}

pub struct RangeProp<T> {
	pub value: T,
	pub min: T,
	pub max: T,
	pub step: T,
}

// impl PropInput<u32> for SliderProp<u32> {
// 	fn get(&self) -> &u32 { &self.value }
// 	fn set(&mut self, value: u32) { self.value = value }
// }

pub struct DropdownProp<T> {
	pub value: T,
	pub index: usize,
	pub display_names: Vec<String>,
}

// impl PropInput<u32> for DropdownProp<u32> {
// 	fn get(&self) -> &u32 { &self.value }
// 	fn set(&mut self, value: u32) { self.value = value }
// }



pub trait UiBuilder<T> {
	fn get_binding(){

	}
}

pub enum BuiltinUiBindings {
	Group,
	Slider,
	Dropdown,
}


pub enum BuiltinUiComponents {}






/*
1. example implementation

struct Foobar {
	a:u32,
	b:u32,
}

impl PropList<Foobar> for Foobar {
	fn get_props() -> Vec<Box<dyn Prop<Foobar>>> {
		vec![
			Box::new(PropHandle {
				get: |prop: &Foobar| prop.a,
			}),
			Box::new(PropHandle {
				get: |prop: &Foobar| prop.b,
			}),
		]
	}
}

fn SliderComponent<T:AcceptsSlider>(entity:Entity){



}




trait PropInput<T>{


}


*/