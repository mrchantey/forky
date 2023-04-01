use bevy::utils::HashMap;

#[derive(Debug, Default, Clone)]
pub struct IdHashMap<T> {
	pub next_id: u64,
	pub map: HashMap<u64, T>,
}

impl<T> IdHashMap<T> {
	pub fn new() -> Self {
		Self {
			next_id: 0,
			map: HashMap::<u64, T>::new(),
		}
	}

	pub fn insert_next(&mut self, value: T) -> (u64, &mut T) {
		let id = self.next_id;
		self.map.insert(id, value);
		self.next_id += 1;
		let value = self.get_mut(&id).unwrap();
		(id, value)
	}
}

impl<T> std::ops::Deref for IdHashMap<T> {
	type Target = HashMap<u64, T>;
	fn deref(&self) -> &Self::Target { &self.map }
}
impl<T> std::ops::DerefMut for IdHashMap<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.map }
}
