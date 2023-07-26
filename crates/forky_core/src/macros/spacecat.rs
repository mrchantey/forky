#[macro_export]
macro_rules! spacecat {
	($single_item:expr) => {
			$single_item
	};
	($first_item:expr, $($rest:tt)*) => {
			concat!($first_item, " ", spacecat!($($rest)*))
	};
}
