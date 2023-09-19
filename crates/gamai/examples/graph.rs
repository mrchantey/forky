#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
//this example is used for macro expansion, for usage see the `tests` directory


#[gamai::node_system]
fn my_node<F: gamai::AiNode>() {}

fn main() {
	let out = gamai::graph!(
		<my_node edge=bar>
		<edgesss/>
		<edge/>
		</my_node>
	);

	println!("{:?}", out);
}
