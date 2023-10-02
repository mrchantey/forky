use bevy_ecs::component;
use bevy_ecs::prelude::*;
use gamai::*;
use std::marker::PhantomData;
use sweet::*;


struct Bazz<T: IntoNodeSystem<M>, M> {
	pub val: T,
	phantom: PhantomData<M>,
}
impl<T: IntoNodeSystem<M>, M> Bazz<T, M> {
	fn foo<A: AiNode>(self, schedule: &mut Schedule, set: impl SystemSet) {
		self.val.into_node_system::<A>(schedule, set)
	}
}

fn foobar<M>(_val: impl IntoNodeSystem<M>) {}

#[derive(Component)]
struct Foo;

fn baz(query: Query<&Foo>) {}

#[sweet_test]
fn works() -> Result<()> {
	foobar(baz);
	Ok(())
}

// sweet! {
// 	it "works" {
// 		expect(true).to_be_false()?;

// 	}
// }
#[allow(non_camel_case_types)]
pub struct my_node;
pub fn my_node_func<N: AiNode>() {}
impl IntoNodeSystem<Self> for my_node {
	fn into_node_system<A: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		schedule.add_systems(my_node_func::<A>.in_set(set));
	}
}
