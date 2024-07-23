use bevy::prelude::*;
use forky_play::mithril;

#[rustfmt::skip]
fn main() { 
	App::new()
		.add_plugins(mithril::MithrilPlugin)
		.run(); 
}
