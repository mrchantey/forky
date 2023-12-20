use anyhow::Result;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_mod_debugdump::schedule_graph::Settings;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct ScheduleGraphSvg;

impl ScheduleGraphSvg {
	pub fn write_default(app: &mut App) -> Result<()> {
		Self::write(app, Update, Path::new("target/graph/graph_out.dot"))
	}
	/// don't add default plugins, this will be done for you
	pub fn write(
		app: &mut App,
		schedule: impl ScheduleLabel,
		path: &Path,
	) -> Result<()> {
		app.add_plugins(
			DefaultPlugins
				.build()
				.disable::<LogPlugin>()
				.disable::<WinitPlugin>(),
		);

		let graph =
			bevy_mod_debugdump::schedule_graph_dot(app, schedule, &Settings {
				// include_system: Some(Box::new(|_| false)),
				..default()
			});

		// let path = "target/graph";
		if let Some(parent) = path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		let mut file = File::create(path).unwrap();
		file.write_all(graph.as_bytes()).unwrap();
		Ok(())
	}
}
