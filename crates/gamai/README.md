# gamai

<div align="center">

  <p>
    <strong>An ECS task switching library suitable for game AI & robotics</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/gamai"><img src="https://img.shields.io/crates/v/gamai.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/gamai"><img src="https://img.shields.io/crates/d/gamai.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/gamai"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://mrchantey.github.io/forky/docs/gamai">Book</a>
    <span> | </span>
    <a href="https://docs.rs/gamai">API Docs</a>
    <span> | </span>
    <a href="https://mrchantey.github.io/forky/docs/other/contributing.html">Contributing</a>
  </h3>

  <sub>made with ❤️‍🔥 by mrchantey</a></sub>
</div>

## Usage

```rs
#[tree_builder]
pub fn MyTree() -> impl TreeElement {
	tree! {
		<sequence>
			<say_hello/>
			<say_world/>
		</sequence>
	}
}

#[action]
fn say_hello<Node: AiNode>(mut query: Query<&mut ActionResult<Node>>){
	
	for mut state in query.iter_mut(){
		println!("hello");
		//tell parent it can go to the next node now
		**state = ActionResult::Success;
	}
}
```