# Gamai

`gamai` is a flexible task switching library suitable for game AI, robotics etc.

```rs
gamai::tree!{
	<select_first_valid_child>
		<say_hello edge=if_framecount_is_two/>
	</select_first_valid_child>
}
```
## Features

- 🔥 Parallel
- 🐢 Systems all the way down
- 🕒 Automatic Scheduling
- ✍️ No Blackboard
- 🌈 Multi-paradigm
- 🌍 With or without Bevy
- 🌴 Digraph architecture

## Overview

The primitive of `gamai` is the `node`, which may execute some behaviour or decide which child should run. Nodes are composed together using two fundamental concepts:
- [generic systems](#generic-systems)
- [compile-time trees](#compile-time-trees)

> Vocabulary from **graph theory** is used to reflect the unopinionated nature of `gamai`, here are some synonyms:
> - Node: `Action, Behaviour, Reasoner, Thinker`
> - Edge: `Consideration, Condition, Filter, Sensor, Scorer`

### Generic Systems

a `node_system` is a bevy system with a single `AiNode` generic argument which allows us to reuse the system in any context. Note the use of the generic parameter in queries:


### Example Action
An action can use the `NodeState` component to check whether it should run, and inform its parent of the current state.
```rs
// nodes run if their parent lets them
#[node_system]
fn child<Node: AiNode>(mut query: Query<&mut NodeState<Node>>){
	for mut state in query.iter_mut(){
		println!("this node is running, its state is {}", state);
		**state = NodeState::Success;
	}
}
```

### Example Reasoner
The parent node is also concerned about the state of its children. In this example it will select the first child that has a passing edge state.
```rs
#[node_system]
fn parent<Node: AiNode>(mut commands: Commands, mut query: Query<Node::ChildQuery>) {
	for (entity, edges) in Node::child_states(&mut query).iter() {
		for (index, edge) in edges.iter().enumerate() {
			if *edge == EdgeState::Pass {
				Node::set_child_node_state(&mut commands, *entity, index).unwrap();
				continue;
			}
		}
	}
}
```
Note the `Node::edges` function that converts a tuple query of `(Edge1,Edge2,Edge3)` into a `Vec<EdgeState>`.


### Compile-time Trees

Once we have our `node_system` primitives we can define our tree using rsx syntax:

```rs
type MyTree = gamai::tree!{
	<parent>
		<child/>
	</parent>
}
```
This does three things:
- Resolve generics in systems for their assigned node
- Specify system execution order
- Create component bundles for each tree

### Running

`gamai::tree!` does two things to get us up and running:
- expose a `build` function that, like a bevy plugin, will schedule our systems
- implements `Bundle` for the tree, so it can be easily spawned

This uses bevy, see [no_bevy](./no_bevy) for more examples.
```rs
fn main(){
	let mut app = App::new();
	app.add_plugins(MyTree::plugin());
	app.world.spawn(MyTree::bundle());
	app.update();
}

// "this node is running, its state is NodeState::Running"
```

> The reason we have a build function instead of a plugin is that `gamai` only depends on `bevy_ecs`