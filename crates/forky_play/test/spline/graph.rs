use petgraph::graph::Graph;
use petgraph::graphmap::UnGraphMap;
use sweet::*;

//https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/
sweet! {
	it "works" {
		let mut graph = Graph::<(), ()>::new(); // directed and unlabeled

		graph.extend_with_edges(&[ (0, 1) ]);

		expect(graph.node_count()).to_be(2)?;
		expect(graph.edge_count()).to_be(1)?;

	}

	it "works 2" {
		let mut graph = Graph::new();
		let origin = graph.add_node("Denver");
		let destination_1 = graph.add_node("San Diego");
		let destination_2 = graph.add_node("New York");
		let cost_1 = graph.add_edge(origin, destination_1, 250);
		let cost_2 = graph.add_edge(origin, destination_2, 1099);

		expect(*graph.node_weight(origin).unwrap()).to_be("Denver")?;
		expect(graph[destination_1]).to_be("San Diego")?;
		expect(*graph.edge_weight(cost_1).unwrap()).to_be(250)?;
		expect(*graph.edge_weight(cost_2).unwrap()).to_be(1099)?;
	}

	test "graph map"{
		let mut graph = UnGraphMap::new();
		let origin = graph.add_node("Denver");
		let dest = graph.add_node("San Diego");
		graph.add_edge(origin, dest, 250);

		expect(graph.edges(origin).count()).to_be(1)?;
		expect(graph.edges(origin).next().unwrap().0).to_be(origin)?;
		expect(graph.edges(origin).next().unwrap().1).to_be(dest)?;

		expect(graph.edges(dest).next().unwrap().0).to_be(dest)?;
		expect(graph.edges(dest).next().unwrap().1).to_be(origin)?;

	}
}
