
// pub trait GenericX<T> {
// 	fn or_default(&self)->T;
// }

// impl GenericX<&str> for Option<&str>{
// 	fn or_default(&self)->&str{
// 		match &self {
// 			Some(c) => c,
// 			None => "",
// 		}
// 	}
// }