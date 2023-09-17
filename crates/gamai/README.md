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


#[agent] //creates MyAgentPlugin & MyAgentBundle
struct MyAgent;

fn main() {
	let will_skip = ChoiceBuilder::new(factor_always_fail, action_print);
	let will_run = ChoiceBuilder::new(factor_always_pass, action_print);

  let mut app = App::new()
  app.add_plugins(MyAgentPlugin::new(default_solver, (will_skip, will_run)));
  app.world.spawn(MyAgentBundle::default());
  app.run();
}

// outputs: "ran action for MyAgent at index 1"
```