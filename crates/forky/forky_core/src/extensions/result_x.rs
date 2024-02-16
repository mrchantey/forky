use extend::ext;

// #[ext]
// pub impl<T, E: Display> Result<T, E> {
// Consume the error, log it and return None, otherwise return the value.
// fn log_err(self) -> Option<T> {
// 	match self {
// 		Ok(value) => Some(value),
// 		Err(err) => {
// 			log::error!("{err}");
// 			None
// 		}
// 	}
// }
// }

#[ext]
pub impl<T, E> Result<T, E> {
	fn ok_or(self, func: impl FnOnce(E)) -> Option<T> {
		match self {
			Ok(value) => Some(value),
			Err(err) => {
				func(err);
				None
			}
		}
	}
}
