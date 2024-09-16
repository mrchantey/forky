//https://danielkeep.github.io/tlborm/book/pat-repetition-replacement.html

/// Container for macros
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
macro_rules! dir {
	($($x:expr) +) => {
		$(print!("{:#?}",$x));+;
		print!("\n");
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
