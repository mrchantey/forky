use gamai::*;
use sweet::*;

// fn foo() {}

#[action]
fn bar<N: AiNode>() {}


#[sweet_test]
fn compiles() -> Result<()> {
	// let _ = tree! {
	// 	<foo>
	// 		<bar/>
	// 	</foo>
	// };

	Ok(())
}
