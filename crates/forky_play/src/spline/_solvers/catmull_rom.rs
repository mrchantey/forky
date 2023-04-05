use bevy::prelude::*;


pub struct CatmullRom;

// const TENSION: f32 = 0.5;

impl CatmullRom {
	pub fn solve_three(node: Vec3, prev: Vec3, next: Vec3) -> (Vec3, Vec3) {
		let tangent = next - prev;
		let dist = tangent.length();
		let dir = tangent.normalize();
		(node - dir * dist, node + dir * dist)
	}
	pub fn solve_two(node: Vec3, next: Vec3) -> Vec3 {
		let tangent = next - node;
		let dist = tangent.length();
		let dir = tangent.normalize();
		node + dir * dist
	}
}
