//this example is used for macro expansion, for usage see the `tests` directory

fn main() {}

#[gamai::node_system]
pub fn my_system<N: gamai::AiNode>() {}
