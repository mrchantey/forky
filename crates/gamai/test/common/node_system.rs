use gamai::*;
use sweet::*;

fn foo() {}

#[node_system]
fn bar<N: AiNode>() {}


#[sweet_test]
fn compiles() -> Result<()> {
	let _ = tree! {
		<foo>
			<bar/>
		</foo>
	};

	Ok(())
}
