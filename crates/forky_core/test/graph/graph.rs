use forky_core::{graph::*, *};
use sweet::*;

sweet! {
	it "works" {

		let mut graph = NodeGraph::from_len(3);

		graph.link(0, 1);

		expect(graph[0].links.contains(&1)).to_be_true()?;

	}
}
