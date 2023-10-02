// // use bevy_ecs::prelude::*;
// use bevy_app::prelude::*;
// use gamai::*;
// use sweet::*;


// // fn foobar(){

// // }

// type MyTree = gamai::tree!(
// 	<sequence>
// 		<print_on_run before=(gamai::print_on_run,gamai::print_on_run)/>
// 		<print_on_run/>
// 		// <foobar/>
// 		// <node_always_succeed/>
// 		// <node_always_succeed/>
// 	</sequence>
// );
// type Child0 = <MyTree as NamedChildren2>::Child0;
// type Child1 = <MyTree as NamedChildren2>::Child1;

// sweet! {

// 	test "cleanup" {

// 		let mut app = App::new();
// 		app.add_plugins(MyTree::plugin());
// 		let entity = app.world.spawn(MyTree::bundle()).id();

// 		app.update();

// 		// expect(&app)
// 		// 	.to_have_component::<DerefNodeState<Child0>>(entity)?;
// 		// expect(&app).not()
// 		// 	.to_have_component::<DerefNodeState<Child1>>(entity)?;

// 		// app.update();

// 		// expect(&app).not()
// 		// 	.to_have_component::<DerefNodeState<Child0>>(entity)?;
// 		// expect(&app)
// 		// 	.to_have_component::<DerefNodeState<Child1>>(entity)?;
// 	}
// }
