//this example is used for macro expansion, for usage see the `tests` directory
use gamai::node::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	type Root = TreePathRoot<0>;
	let tree = || {
		let a = Node0::<Root, _>::new(DefaultAttributes::default());
		Node1::<Root, _, _>::new(DefaultAttributes::default(), a).into_root()
	};

	let _ = AiPlugin::new(tree);
	let _ = AiPlugin::new(tree);

	Ok(())
}
