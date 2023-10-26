use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct PoseLerp {
	pub timer: Timer,
	pub origin: Pose,
	pub target: Pose,
}

#[rustfmt::skip]
pub fn pose_lerp_animator(
	time: Res<Time>,
	mut commands:Commands,
	mut query: Query<(Entity,&mut Transform, &mut PoseLerp)>,
) {
	
	for (entity,mut tran,mut pose_lerp) in query.iter_mut() {
		// let mut timer = &;
		if pose_lerp.timer.tick(time.delta()).just_finished() {
			commands.entity(entity)
				.remove::<PoseLerp>();
		} else if pose_lerp.timer.finished() {
			continue;
		}else{
			let t = pose_lerp.timer.elapsed_secs() / pose_lerp.timer.duration().as_secs_f32();
			let lerped = Pose::lerp(&pose_lerp.origin,&pose_lerp.target,t);
			tran.translation = lerped.position;
			tran.rotation = lerped.rotation;
		}
	}
}
