use bevy::app::AppExit;
use bevy::prelude::*;
// use enigo::*;

pub fn spawn_camera_at_position(position: Vec3) -> impl FnMut(Commands) {
	move |mut commands| {
		commands.spawn(Camera3dBundle {
			transform: Transform::from_translation(position)
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		});
	}
}

pub fn spawn_default_lights(mut commands: Commands) {
	commands.insert_resource(AmbientLight {
		// color: Color::WHITE,
		brightness: 1.,
		..default()
	});

	commands.spawn((Name::new("Left Light"), PointLightBundle {
		transform: Transform::from_xyz(-5., 5., 3.),
		point_light: PointLight {
			intensity: 1000.,
			// color: Color::FUCHSIA,
			shadows_enabled: true,
			..default()
		},
		..default()
	}));
	commands.spawn((Name::new("Right Light"), PointLightBundle {
		transform: Transform::from_xyz(3., 5., -5.),
		point_light: PointLight {
			intensity: 1000.,
			shadows_enabled: true,
			// color: Color::CYAN,
			..default()
		},
		..default()
	}));
}


pub fn create_exit_after_system(
	secs: f64,
) -> impl Fn(EventWriter<AppExit>, Res<Time>) {
	move |mut exit, time| {
		if time.elapsed_seconds_f64() > secs {
			exit.send(AppExit);
		}
	}
}

pub fn exit_system(mut exit: EventWriter<AppExit>) { exit.send(AppExit); }

pub fn surrender_focus() {
	// let mut enigo = Enigo::new();
	// enigo.key_down(Key::Alt);
	// enigo.key_click(Key::Escape);
	// enigo.key_up(Key::Alt);
}
