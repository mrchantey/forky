//https://danielkeep.github.io/tlborm/book/pat-repetition-replacement.html
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
#[macro_export]
macro_rules! log {
	($($x:expr) +) => {
		$(print!("{}",$x));+;
		print!("\n");
	};
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


// macro_rules! fmt {
// 	($($x:expr) +) => {
// 		[$(replace_expr!($x "{}")),+].join("baz")
// 	};
// }
