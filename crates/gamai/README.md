# gamai

<div align="center">

  <p>
    <strong>A flexible & extendable game AI library.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/gamai"><img src="https://img.shields.io/crates/v/gamai.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/gamai"><img src="https://img.shields.io/crates/d/gamai.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/gamai"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://mrchantey.github.io/forky/docs/gamai">Guide</a>
    <span> | </span>
    <a href="https://docs.rs/gamai">API Docs</a>
    <span> | </span>
    <a href="https://mrchantey.github.io/forky/docs/other/contributing.html">Contributing</a>
  </h3>

  <sub>made with ❤️‍🔥 by mrchantey</a></sub>
</div>

## Usage

```rs


#[node] //creates MyAiNodePlugin & MyAiNodeBundle
struct MyNode;

fn main() {
	let will_skip = EdgeBuilder::new(edge_always_fail, print_on_run);
	let will_run = EdgeBuilder::new(edge_always_pass, print_on_run);

  let mut app = App::new()
  app.add_plugins(MyNodePlugin::new(default_system, (will_skip, will_run)));
  app.world.spawn(MyNodeBundle::default());
  app.run();
}

// outputs: "ran child_node for MyAiNode at index 1"
```