# Gamai

`gamai` is a flexible task switching library suitable for game AI, robotics etc.

## Features

- 🔥 Parallel
- ✍️ No Blackboard
- 🌴 Digraph architecture
- 🌈 Unopinionated
- 🌍 With or without Bevy
- 🐢 Systems all the way down

## Overview

`gamai` has two fundamental concepts:
- [generic systems](#generic-systems)
- [compile-time trees](#compile-time-trees)


> Vocabulary from graph theory is used in `gamai`, here are some synonyms:
> - Node: Action, Behaviour
> - Edge: Consideration, Filter, Sensor, Scorer

### Generic Systems

If we want parallelism then every node and edge should only mutably access what it needs and not block its siblings. For code reuse we can use a generic system, this way our systems dont need to know which node they belong to.

Note the use of the `Node` parameter to specify the required state for all three systems:
```rs

// edges give their parent an idea of whether their node should run
#[node]
fn child_edge<Node: AiNode>(query: Query<&mut EdgeState<Node>>){
	for state in query.iter_mut(){
			**state = EdgeState::Weight(0.7);
	}
}

// nodes run if their parent lets them
#[node]
fn child_node<Node: AiNode>(query: Query<&NodeState<Node>>){
	for state in query.iter(){
		println!("this node is running, its state is {}", state);
	}
}
```

The parent node is a little more complex, it uses the generic parameter to retrieve all child states:
```rs
// parents decide which children get to run, based on their edge states
#[node]
fn parent<Node: AiNode>(mut commands: Commands, mut query: Query<Node::ChildrenQuery>) {
	let entities = Node::edges(&mut query); // Vec(Entity,Vec<EdgeState>)
	for (entity, edges) in entities.iter() {
		for (index, edge) in edges.iter().enumerate() {
			if *edge != EdgeState::Fail {
				Node::set_child_node_state(&mut commands, *entity, index).unwrap();
				continue;
			}
		}
	}
}

```

### Compile-time Trees

The above example is nice and modular, but a little unweildy for use. There are three more things to be done:
- Resolve generics for their tree
- Specify system execution order
	- child edges should run before parents, and child nodes after parents
- Create component bundles for each tree

The below example uses bevy but see [no_bevy](./no_bevy) for more examples.
```rs
type MyTree = gamai::tree!(
	<parent>
		<child_node edge=child_edge/>
	</parent>
)

fn main(){
	let mut app = App::new();
	// like a plugin, but gamai only uses bevy_ecs
	MyTree::build(app.world.schedule(Update).unwrap());
	// its a bundle :)
	app.world.spawn(MyTree::default());
	app.update();
}

// "this node is running, its state is NodeState::Once"
```