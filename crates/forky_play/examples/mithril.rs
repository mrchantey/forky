use bevy::prelude::*;
use forky_play::mithril;

#[rustfmt::skip]
fn main() { 
	App::new()
		.add_plugin(mithril::MithrilPlugin)
		.run(); 
}
