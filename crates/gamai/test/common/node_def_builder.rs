//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;
use sweet::*;

#[action]
fn my_system<N: AiNode>() {}

#[sweet_test]
pub fn works() -> Result<()> {
	let el = || {
		ParentElement1::<TreePathRoot<0>, _, _, _>::new(
			my_system.into_action_config(),
			my_system,
			ParentElement0::<TreePathRoot<0>, _, _>::new(
				my_system.into_action_config(),
				my_system,
			),
		)
		.into_root()
	};

	// let tree1 = || tree! {<my_system/>};
	let _bundle = TreeBundle::new(el);
	let _bundle = TreeBundle::new(el);
	Ok(())
}
