use gamai::node::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let el = || ParentElement0::<TreePathRoot<0>, _, _>::new((), ());

	let expected:&str = "gamai::node::node_def::ParentElement0<gamai::node::tree_path::TreePathRoot<0>, (), ()>";

	expect(std::any::type_name_of_val(&el())).to_be(expected)?;
	expect(std::any::type_name_of_val(&el().into_root())).to_be(expected)?;
	expect(std::any::type_name_of_val(&el().into_root().into_root()))
		.to_be(expected)?;


	// let el = || {
	// 	tree! {
	// 		<group>
	// 			<group/>
	// 		</group>
	// 	}
	// };
	// expect(std::any::type_name_of_val(&el())).to_be(expected)?;


	Ok(())
}
