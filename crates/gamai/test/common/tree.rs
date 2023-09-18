use bevy::prelude::*;
use gamai::*;
use sweet::*;

type F<const I: usize> = ChoiceEdgeState<ChoicePhantom<MyAiNode, I>>;
type A<const I: usize> = ChoiceActionState<ChoicePhantom<MyAiNode, I>>;


// #[node(32)]
#[node]
struct MyAiNode;

sweet! {
	it "works" {
		let choice0 = ChoiceBuilder::new(edge_always_fail, action_noop);
		let choice1 = ChoiceBuilder::new(edge_always_pass, action_noop);
		// let choice0 = (edge_always_fail, action_print);
		// let choice1 = (edge_always_pass, action_print);
		let mut app = App::new();
		let entity = app.world.spawn(MyAiNodeBundle::default()).id();
		expect(&app).not().to_have_component::<A<0>>(entity)?;
		expect(&app).to_have_component::<F<0>>(entity)?;
		app.add_plugins(MyAiNodePlugin::new(solver_first_valid, (choice0, choice1)));
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
