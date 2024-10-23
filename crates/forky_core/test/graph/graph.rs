use forky_core::prelude::*;
use sweet::*;

#[sweet_test]
fn link()-> Result<()> {
	let mut graph = NodeGraph::from_len(2);
	graph.link(0, 1);	
	expect(graph[0].links.contains(&1)).to_be_true()?;
	
	Ok(())
}

#[sweet_test(skip)]
fn remove()-> Result<()> {
	let mut graph = NodeGraph::from_len(3);
	
	graph.link(0, 1);
	graph.link(0, 2);
	
	expect(graph[2].links.contains(&0)).to_be_true()?;
	graph.remove(0);
	
	expect(graph[2].links.contains(&0)).to_be_false()?;
	
	Ok(())
}
