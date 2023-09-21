# Edges (utility ai)

```rs
// edges give their parent an idea of whether their node should run
#[node_system]
fn child_edge<Node: AiNode>(query: Query<&mut EdgeState<Node>>){
	for state in query.iter_mut(){
			**state = EdgeState::Weight(0.7);
	}
}
```

```rs
type MyTree = gamai::tree!{
	<parent>
		<child edge=child_edge/>
	</parent>
}
```

## Scheduling

Gamai schedules child edges to run before parents, and child nodes after parents, giving the parent up-to-date information about its children when it runs.