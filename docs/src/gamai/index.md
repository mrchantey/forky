# Gamai

Gamai is an ECS task switching library suitable for game AI, robotics & other performance-critical environments. The primitives it provies can be used for multiple decision-making paradigms like Behaviour Trees, Utility AI and Goal Oriented Action Planning (GOAP). The ECS implementation uses opportunistic parallelism, ensuring trees are processed as quickly as possible.

**With Bevy**

If used with Bevy there is no blackboard, each node is a regular Bevy system with the same access to entities & resources.

**Without Bevy**

The lightweight [`bevy_ecs`][1] crate that drives Gamai has a great storage pattern, scheduling systems to safely run in parallel and storing data efficiently for the CPU cache.

## Features

- 🌴 Composable Tree Definitions
- 🔥 Compile-time Parallel Optimization
- ✍️ No Blackboard
- 🌈 Multi-paradigm
- 🌍 With or without Bevy


## Overview

Gamai has three fundamental concepts: `Props`, `Actions` & `Trees`.

### Props

A `Prop` is a regular bevy Component with an added `AiNode` generic argument, meaning the same prop can be used to represent the state of individual nodes in the tree.

For instance the `Running` prop is used to indicate whether an action is currently running.

### Actions

An `action` is a bevy systems with an added generic `AiNode` argument which can be used to access props and children:
```rs
#[action]
fn say_hello<N: AiNode>(query: Query<Entity, With<Prop<Running,N>>){	
	for _ in query.iter(){
		println!("this action is running!");
	}
}
```

### Trees

Trees are defined using the same proven RSX pattern used in web UI libraries. Each node can be either an action or a sub-tree.

```rs
#[tree_builder]
pub fn MyTree() -> impl AiNode {
	tree! {
		<sequence>
			<say_hello/>
			<SayWorld/> //a tree declared elsewhere
		</sequence>
	}
}
```

> `gamai` uses a naming convention like web UI libraries:
> - `actions` have snake_case
> - `trees` have PascalCase


## Running

Running a tree requires setting up of state and systems:
- A `TreePlugin` schedules all systems in the tree:
  
	```rust
	app.add_plugins(TreePlugin::new(MyTree));
	```
- A `TreeBundle` will add given props to specified nodes in the tree.
	```rust
	// only set the root as running
	app.world.spawn(TreeBundle::root(MyTree, Running));
	// set all nodes in the tree to have a failing score
	app.world.spawn(TreeBundle::recursive(MyTree, Score::Fail));
	```

Putting it all together:

```rs
fn main(){
	let mut app = App::new();	
	app.add_plugins(TreePlugin::new(MyTree));
	app.world.spawn(TreeBundle::root(MyTree, Running));

	app.update(); // runs first child
	app.update(); // runs second child
}
```
```sh
> cargo run
hello
world
```
<!-- > This example uses `bevy`, see [no_bevy](./no_bevy) for more examples. -->

[1]: https://crates.io/crates/bevy_ecs
