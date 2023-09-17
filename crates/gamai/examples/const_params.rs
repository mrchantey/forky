#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy::prelude::*;
use std::marker::PhantomData;
use sweet::*;

// #[derive(Component)]
#[derive(Default)]
struct MyAgent;

impl AgentType for MyAgent {}

#[derive(Default, Component)]
struct AgentRef<Choice: Foo> {
	agent: PhantomData<Choice>,
}

trait AgentType: Default {}

trait Foo: Default {
	const INDEX: usize;
	type Agent: AgentType;
}
#[derive(Default)]
struct FooStruct<A: AgentType, const I: usize> {
	agent: PhantomData<A>,
}
impl<A: AgentType, const I: usize> Foo for FooStruct<A, I> {
	const INDEX: usize = I;
	type Agent = A;
}

// impl<T: AgentType,const I:usize> Foo for AgentRef<T> {
// 	const INDEX: usize = 0;
// 	type Agent = T;
// }

fn main() -> anyhow::Result<()> {
	let mut app = App::new();

	let entity = app
		.world
		.spawn((
			AgentRef::<FooStruct<MyAgent, 0>>::default(),
			AgentRef::<FooStruct<MyAgent, 1>>::default(),
		))
		.id();

	expect(&app)
		.to_have_component::<AgentRef<FooStruct<MyAgent, 0>>>(entity)?;
	expect(&app)
		.to_have_component::<AgentRef<FooStruct<MyAgent, 1>>>(entity)?;
	expect(&app)
		.not()
		.to_have_component::<AgentRef<FooStruct<MyAgent, 2>>>(entity)?;
	Ok(())
}
