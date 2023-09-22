// use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use gamai::*;
use sweet::*;

type MyTree = gamai::tree!(
	<sequence>
		<node_always_succeed/>
		<node_always_succeed/>
		// <print_on_run edge=edge_always_pass/>
	</sequence>
);
type Child0 = <MyTree as NamedChildren2>::Child0;
type Child1 = <MyTree as NamedChildren2>::Child1;

sweet! {

	test "sequence" {
		let mut app = App::new();
		app.add_plugins(MyTree::plugin());
		// let mut world = World::new();
		// let mut schedule = Schedule::default();
		// MyTree::add_systems(&mut schedule);
		let entity = app.world.spawn(MyTree::bundle()).id();

		app.update();

		expect(&app)
			.to_have_component::<DerefNodeState<Child0>>(entity)?;
		expect(&app).not()
			.to_have_component::<DerefNodeState<Child1>>(entity)?;

		app.update();

		expect(&app).not()
			.to_have_component::<DerefNodeState<Child0>>(entity)?;
		expect(&app)
			.to_have_component::<DerefNodeState<Child1>>(entity)?;
	}
}
