use bevy::prelude::*;
use gamai::*;
use sweet::*;

type F<const I: usize> = ChildEdgeState<EdgePhantom<MyAiNode, I>>;
type A<const I: usize> = ChildNodeState<EdgePhantom<MyAiNode, I>>;


// #[node(32)]
#[node]
struct MyAiNode;

sweet! {
	it "works" {
		let edge0 = ChildNodeBuilder::new(edge_always_fail, noop_node);
		let edge1 = ChildNodeBuilder::new(edge_always_pass, noop_node);
		// let edge0 = (edge_always_fail, print_on_run);
		// let edge1 = (edge_always_pass, print_on_run);
		let mut app = App::new();
		let entity = app.world.spawn(MyAiNodeBundle::default()).id();
		expect(&app).not().to_have_component::<A<0>>(entity)?;
		expect(&app).to_have_component::<F<0>>(entity)?;
		app.add_plugins(MyAiNodePlugin::new(first_valid_edge, (edge0, edge1)));
		app.finish();
		app.update();
		expect(&app)
			.component::<F<0>>(entity)?
			.map(|a| a.state)
			.to_be(EdgeState::Fail)?;
		expect(&app)
			.component::<F<1>>(entity)?
			.map(|a| a.state)
			.to_be(EdgeState::Pass)?;
		expect(&app).not().to_have_component::<A<0>>(entity)?;
		expect(&app).to_have_component::<A<1>>(entity)?;
	}
}
