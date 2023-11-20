// use crate::*;
use super::*;

pub trait ActionMeta {
	fn name() -> String;
	fn action() -> impl IntoAction;
}
