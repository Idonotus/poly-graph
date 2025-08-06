use bevy::math::{Vec2, Vec3};

use crate::simulation::shape::{Polyhedron, Vertex};
use std::collections::HashSet;

const UVGRID_SIZE: Vec2 = Vec2 {
	x: 10f32,
	y: 10f32
};
const INV_UVGRID_SIZE:Vec2 = Vec2 {
	x: 1f32 / UVGRID_SIZE.x,
	y: 1f32 / UVGRID_SIZE.y
};

fn get_uv_pos(idx: f32) -> Vec2 {
	return Vec2 {
		x: (idx + 0.5f32) % UVGRID_SIZE.x,
		y: (idx + 0.5f32) / UVGRID_SIZE.x % UVGRID_SIZE.y
	} * INV_UVGRID_SIZE
}

fn mesh_context_from_polyherdron(p: &Polyhedron) {
	let mut v_position: Vec<Vec3> = Vec::new();
	let mut v_normals: Vec<Vec3> = Vec::new();
	let mut trigs: Vec<u32> = Vec::new();
	let mut v_uv_position: Vec<Vec2> = Vec::new();
	
	let vertex_set: &mut HashSet<usize> = &mut HashSet::new();
	for (idx,f) in p.faces.iter().enumerate() {
		for v in &f.vertices {	
			vertex_set.insert(*v);
		}

		let mut totalmass = Vec3::new(0f32, 0f32,0f32);
		let v_count = vertex_set.len();
		let fpoint = v_position.len() as u32;
		let mut lpoint = 0;
		for v in vertex_set.drain() {
			let vertex = &p.vertices[v];
			totalmass += vertex.position;
			v_position.push(vertex.position);
			let curvert = v_position.len() as u32;
			if fpoint + 1 < curvert {
				trigs.push(fpoint);
				trigs.push(lpoint);
				trigs.push(curvert);
			};
			lpoint = curvert
		}
		v_normals.append(&mut vec![(totalmass/v_count as f32).normalize_or_zero()]);
	
		v_uv_position.append(&mut vec![get_uv_pos(idx as f32); v_count]);
	}
}