use bevy_utils::HashMap;
use extend::ext;
use std::hash::Hash;

#[ext]
pub impl<T1, T2> HashMap<T1, T2>
where
	T1: Hash + PartialEq + Eq + Clone,
	T2: Default,
{
	fn get_or_default(&mut self, key: T1) -> &T2 {
		if self.contains_key(&key) {
			return self.get(&key).unwrap();
		}
		self.insert(key.clone(), Default::default());
		self.get(&key).unwrap()
	}
	fn get_or_default_mut(&mut self, key: T1) -> &mut T2 {
		if self.contains_key(&key) {
			return self.get_mut(&key).unwrap();
		}
		self.insert(key.clone(), Default::default());
		self.get_mut(&key).unwrap()
	}
}
