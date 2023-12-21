/// Define an action list. This macro accepts a name and a list of actions.
///
/// ```rust
///
/// action_list!(AgentNodes, [
/// 	Run,
/// 	Hide,
/// 	ChooseWhatToDo
/// ]);
/// ```
///
#[macro_export]
macro_rules! action_list {
	($name:ident, [$($variant:ident),*]) => {
		#[allow(unused_imports)]
		use gamai::prelude::*;
		#[allow(unused_imports)]
		use gamai::exports::*;
		use strum::IntoEnumIterator;
		use strum_macros::EnumIter;
		#[derive(EnumIter, Clone, Serialize, Deserialize)]
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
