//https://danielkeep.github.io/tlborm/book/pat-repetition-replacement.html

/// Container for macros
pub mod macros_inner {
	#[macro_export]
	macro_rules! replace_expr {
		($_t:tt $sub:expr) => {
			$sub
		};
	}

	// #[macro_export]
	// macro_rules! fmt {
	// 	($($x:expr) +) => {
	// 		$(format!("{} ",$x))+
	// 	};
	// }
	// #[macro_export]
	// macro_rules! log {
	// 	($($x:expr) +) => {
	// 		$(print!("{}",$x));+;
	// 		print!("\n");
	// 	};
	// }
	#[macro_export]
	macro_rules! log {
	( $( $t:tt )* ) => {
			#[cfg(not(target_arch = "wasm32"))]
			println!($( $t )*);
			#[cfg(target_arch = "wasm32")]
			web_sys::console::log_1(&format!( $( $t )* ).into());
	}
}

	#[macro_export]
	macro_rules! dir {
	($($x:expr) +) => {
		$(print!("{:#?}",$x));+;
		print!("\n");
	};
}

	#[macro_export]
	macro_rules! tern {
		($pred:expr; $a:expr; $b:expr) => {
			if $pred {
				$a
			} else {
				$b
			}
		};
	}

	#[macro_export]
	macro_rules! spacecat {
	// ($single_item:expr) => {
	// 		$single_item
	// };
	// ($first_item:expr, $($rest:tt)*) => {
	// 		concat!($first_item, " ", spacecat!($($rest)*))
	// };
	( $($string:expr),* ) => {
		vec![ $($string.to_string()),* ].join(" ")
	};
}

	// macro_rules! fmt {
	// 	($($x:expr) +) => {
	// 		[$(replace_expr!($x "{}")),+].join("baz")
	// 	};
	// }
}
