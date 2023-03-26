use bevy::ecs::query;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_play::spline::*;
use forky_play::*;
use sweet::*;
// impl Matchable for Vec3 {}
sweet! {

	test "spline mesh" {

		let spline = Spline::Linear(LinearSpline {
			p0: Vec3::new(0., 0., 0.),
			p1: Vec3::new(0., 0., -10.),
		});

		let edge_loop = spline::mesh::rect_edge_loop(1.,0.1);

		let subdivisions = 1;
		let divisions = subdivisions + 2;


		let vertices = spline::mesh::spline_to_vertices(&spline, &edge_loop,subdivisions);

		expect(spline::mesh::vertex_divisions(&vertices,&edge_loop)).to_be(divisions)?;
		expect(vertices[0]).to_be(Vec3::new(-0.5, 0.05, 0.))?;
		expect(vertices[4 * divisions - 1]).to_be(Vec3::new(0.5, 0.05, -10.))?;

		let triangles = spline::mesh::spline_to_triangles(edge_loop.len(),subdivisions);
		expect(triangles.len()).to_be(edge_loop.len() * (divisions-1) * 6)?;
		expect(&format!("{:?}",triangles)[..]).to_be(&format!("{:?}",vec![0, 1, 4, 1, 5, 4, 1, 2, 5, 2, 6, 5, 2, 3, 6, 3, 7, 6, 3, 0, 7, 0, 4, 7, 4, 5, 8, 5, 9, 8, 5, 6, 9, 6, 10, 9, 6, 7, 10, 7, 11, 10, 7, 4, 11, 4, 8, 11])[..])?;
		// 3, 0, 7, 0, 4, 7

		let uvs = spline::mesh::spline_to_uv(&spline, edge_loop.len(),subdivisions);
		expect(uvs.len()).to_be(vertices.len())?;
		expect(uvs[0].y).to_be(0.)?;
		expect(uvs.last().unwrap().y).to_be(1.)?;

		// pretty_print(&edge_loop,&vertices,&triangles,&uvs);
	}
}


fn pretty_print(
	edge_loop: &Vec<Vec2>,
	vertices: &Vec<Vec3>,
	triangles: &Vec<u32>,
	uvs: &Vec<Vec2>,
) {
	let divisions = spline::mesh::vertex_divisions(&vertices, &edge_loop);
	let edge_loop_len = edge_loop.len();
	println!("edge_loop: \n{:?}", edge_loop);
	println!("\nvertices:");
	for i in 0..divisions {
		println!("\ndivision {i}");
		for j in 0..edge_loop_len {
			let iv = i * edge_loop_len + j;
			println!("{iv}: vertex: {}, uv: {}", vertices[iv], uvs[iv]);
		}
	}
	println!("\ntriangles:");
	for di in 0..divisions - 1 {
		println!("\ndivision {di}");
		for li in 0..edge_loop_len {
			let it = di * edge_loop_len * 6 + li * 6;
			println!(
				"{it}:\t{},{},{}\t{},{},{}",
				triangles[it + 0],
				triangles[it + 1],
				triangles[it + 2],
				triangles[it + 3],
				triangles[it + 4],
				triangles[it + 5],
			);
		}
	}
}
