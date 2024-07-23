use leptos::*;

#[extend::ext]
pub impl WriteSignal<bool> {
	fn toggle(self) { self.update(|prev| *prev = !*prev); }
}
