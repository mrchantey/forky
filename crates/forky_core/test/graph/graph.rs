use forky_core::{graph::*, *};
use sweet::*;

sweet! {
	test "link" {

		let mut graph = NodeGraph::from_len(2);

		graph.link(0, 1);

		expect(graph[0].links.contains(&1)).to_be_true()?;

	}
	//not supported
	test skip "remove" {

		let mut graph = NodeGraph::from_len(3);

		graph.link(0, 1);
		graph.link(0, 2);

		expect(graph[2].links.contains(&0)).to_be_true()?;
		graph.remove(0);

		expect(graph[2].links.contains(&0)).to_be_false()?;

	}
}
