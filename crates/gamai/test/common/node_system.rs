use gamai::*;
use sweet::*;

fn foo() {}

#[node_system]
fn bar<N: AiNode>() {}


#[sweet_test]
fn works() -> Result<()> {
	let _tree = tree! {
		<foo>
			<bar/>
		</foo>
	};

	Ok(())
}
