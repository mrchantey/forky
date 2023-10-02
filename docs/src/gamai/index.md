# Gamai

`gamai` is a flexible task switching library suitable for game AI, robotics & other performance-critical environments. It uses a modular behavior tree structure and supports multiple selector paradigms like Utility AI or Goal Oriented Action Planning (GOAP). The ECS implementation allows for opportunistic parallelism, ensuring trees are processed as quickly as possible.

**With Bevy**

If used with Bevy there is no blackboard, as each node is a regular Bevy system with the same world access.

**Without Bevy**

The lightweight [`bevy_ecs`][1] crate that drives Gamai makes for an excellent blackboard, scheduling nodes to safely run in parallel and storing data efficiently for the CPU cache.

## Features

- 🔥 Automatic Parallelism
- 🌴 ECS Behaviour Tree
- 🐢 Intuitive Definition & Composition
- 🕒 Automatic System Ordering
- ✍️ No Blackboard
- 🌈 Multi-paradigm
- 🌍 With or without Bevy


## Overview

### Nodes

Nodes are systems that accept a single `AiNode` type argument, giving them access to the components that they need to communicate with parents and children. They are used 
to either run some behaviour or decide which child should run.

```rs
fn say_hello<Node: AiNode>(mut query: Query<&mut NodeState<Node>>){
	
	for mut state in query.iter_mut(){
		println!("hello world");

		assert_eq!(**state, NodeState::Running);
		//tell parent it can go to next node now
		**state = NodeState::Success;
	}
}
```
### Trees

Trees are composed in rsx parsed at *compile time* which gives us the nessesary type information for the opportunistic parallel scheduler in `bevy_ecs`.

```rs
type MyTree = gamai::tree!{
	<sequence>
		<say_hello/>
		<say_hello/>
	</sequence>
}
```

### Running

The type returned from `tree!` has two static methods: 
- `MyTree::plugin()` provides the plugin that will add the systems to the world in the correct order.
- `MyTree::bundle()` provides the components nessecary for the tree to run for a given entity.

> This example uses `bevy` , see [no_bevy](./no_bevy) for more examples.

```rs
fn main(){
	let mut app = App::new();
	app.add_plugins(MyTree::plugin());
	app.world.spawn(MyTree::bundle());
	app.update();
}
```
```sh
> cargo run
hello world
hello world
```

[1]: https://crates.io/crates/bevy_ecs
