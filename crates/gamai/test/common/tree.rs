use bevy::prelude::*;
use gamai::*;
use sweet::*;

type F<const I: usize> = ChoiceEdgeState<ChoicePhantom<MyAgent, I>>;
type A<const I: usize> = ChoiceActionState<ChoicePhantom<MyAgent, I>>;


// #[agent(32)]
#[agent]
struct MyAgent;

sweet! {
	it "works" {
		let choice0 = ChoiceBuilder::new(edge_always_fail, action_noop);
		let choice1 = ChoiceBuilder::new(edge_always_pass, action_noop);
		// let choice0 = (edge_always_fail, action_print);
		// let choice1 = (edge_always_pass, action_print);
		let mut app = App::new();
		let entity = app.world.spawn(MyAgentBundle::default()).id();
		expect(&app).not().to_have_component::<A<0>>(entity)?;
		expect(&app).to_have_component::<F<0>>(entity)?;
		app.add_plugins(MyAgentPlugin::new(solver_first_valid, (choice0, choice1)));
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
