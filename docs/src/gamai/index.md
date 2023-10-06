# Gamai

`gamai` is a flexible task switching library suitable for game AI, robotics & other performance-critical environments. It features a modular tree structure and supports multiple selector paradigms like Utility AI and Goal Oriented Action Planning (GOAP). The ECS implementation allows for opportunistic parallelism, ensuring trees are processed as quickly as possible.

**With Bevy**

If used with Bevy there is no blackboard, as each node is a regular Bevy system with the same access to entities & resources.

**Without Bevy**

The lightweight [`bevy_ecs`][1] crate that drives Gamai makes for an excellent blackboard, scheduling nodes to safely run in parallel and storing data efficiently for the CPU cache.

## Features

- 🌴 Declarative RSX Trees
- 🔥 Automatic Parallelism & System Ordering
- ✍️ No Blackboard
- 🌈 Multi-paradigm
- 🌍 With or without Bevy

## Trees

Trees are defined using familiar RSX patterns like those found in web UI libraries. A unique aspect of `gamai` is that trees are parsed at *compile time* which gives us the nessesary type information for the parallel scheduler in `bevy_ecs`.

```rs
#[tree_builder]
pub fn MyTree() -> impl AiTree {
	tree! {
		<sequence>
			<say_hello/>
			<say_world/>
		</sequence>
	}
}
```

> `gamai` uses the same naming conventions as UI libraries like react or yew:
> - `node_systems` have snake_case 
> - `tree_builders` have PascalCase

### Node Systems

Any Bevy system can be used as a node, but usually we want to know whether it is in a running state for a given entity. Node systems are bevy systems that also accept an `AiNode` type argument, giving them access to the components that they need to communicate with parents and children.

Node Systems can be used to either run some behaviour (actions) or decide which child should run (selectors).

```rs
#[node_system]
fn say_hello<Node: AiNode>(mut query: Query<&mut NodeState<Node>>){
	
	for mut state in query.iter_mut(){
		println!("hello");
		assert_eq!(**state, NodeState::Running);
		//tell parent it can go to the next node now
		**state = NodeState::Success;
	}
}
```

### Running

An `AiNode` has two methods for running:
- `MyTree.plugin()` provides the plugin that will add the systems to the world in the correct order.
- `MyTree.bundle()` provides the components nessecary for the tree to run for a given entity.


```rs
fn main(){
	let mut app = App::new();
	app.add_plugins(MyTree.plugin());
	app.world.spawn(MyTree.bundle());
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
