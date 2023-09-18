#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy::prelude::*;
use gamai::*;

//this example is used for macro expansion, for usage see the `tests` directory
fn main() {}

#[node_system]
fn first_valid_solver<N: AiNode>(
	mut commands: Commands,
	mut query: Query<N::ChildrenQuery>,
) {
	let entities = N::edges(&mut query);
	for (entity, edges) in entities.iter() {
		for (index, edge) in edges.iter().enumerate() {
			if *edge != EdgeState::Fail {
				N::set_child_node_state(&mut commands, *entity, index);
				return;
			}
		}
	}
}
