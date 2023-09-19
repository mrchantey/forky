#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
//this example is used for macro expansion, for usage see the `tests` directory

fn main() {}

gamai::node!(1);
// gamai::node!(1);
// #[gamai::node(num_edges = 0)]
// #[gamai::node(num_choices = 20)]
// #[gamai::node(num_edges(20))]
// fn my_node() {}
// fn my_node<A: gamai::AiNode>() {}

// #[node]
// fn first_valid_edge<N: AiNode>(
// 	mut commands: Commands,
// 	mut query: Query<N::ChildrenQuery>,
// ) {
// 	let entities = N::edges(&mut query);
// 	for (entity, edges) in entities.iter() {
// 		for (index, edge) in edges.iter().enumerate() {
// 			if *edge != EdgeState::Fail {
// 				N::set_child_node_state(&mut commands, *entity, index);
// 				return;
// 			}
// 		}
// 	}
// }
