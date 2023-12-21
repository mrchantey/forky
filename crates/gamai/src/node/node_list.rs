/// Define a node list. This macro accepts a name and a list of actions.
///
/// ```rust
///
/// node_list!(AgentNodes, [
/// 	Run,
/// 	Hide,
/// 	ChooseWhatToDo
/// ]);
/// ```
///
#[macro_export]
macro_rules! node_list {
	($name:ident, [$($variant:ident),*]) => {
		#[allow(unused_imports)]
		use gamai::prelude::*;
		#[allow(unused_imports)]
		use gamai::exports::*;
		#[derive(Clone, Serialize, Deserialize)]
		pub enum $name {
			$($variant($variant),)*
		}

		impl IntoAction for $name {
			fn into_action(self) -> Box<dyn Action> {
				match self {
					$(Self::$variant(x) => Box::new(x),)*
				}
			}
			fn into_action_ref(&self) -> &dyn Action {
				match self {
					$(Self::$variant(x) => x,)*
				}
			}
			fn into_action_mut(&mut self) -> &mut dyn Action {
				match self {
					$(Self::$variant(x) => x,)*
				}
			}
		}
	};
}
