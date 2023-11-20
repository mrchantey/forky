// use crate::*;
use crate::node::IntoAction;



pub trait ActionMeta {
	fn name() -> String;
	fn action() -> impl IntoAction;
}
