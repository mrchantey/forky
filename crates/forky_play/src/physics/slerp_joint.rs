use crate::{QuatExt, Vec3Ext};
use bevy::prelude::*;

#[derive(Resource)]
struct ChangeTargetTimer(Timer);


pub struct SlerpJointPlugin;


#[derive(Component, Default)]
pub struct SlerpTarget {
	target: Quat,
}


impl Plugin for SlerpJointPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ChangeTargetTimer(Timer::from_seconds(
			1.,
			TimerMode::Repeating,
		)))
		// .add_startup_system(add_people)
		.add_system(update_slerp);
	}
}


fn update_slerp(
	time: Res<Time>,
	mut timer: ResMut<ChangeTargetTimer>,
	mut query: Query<(&mut Transform, &mut SlerpTarget)>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (_, mut slerp) in query.iter_mut() {
			slerp.target = Quat::look_at(Vec3::random_on_sphere())
		}
	}
	let d = time.delta_seconds();
	for (mut tran, target) in query.iter_mut() {
		// tran.rotation = Quat::look_at(Vec3::random_on_sphere());
		tran.rotation.rotate_towards(target.target, 1. * d);
	}
}
