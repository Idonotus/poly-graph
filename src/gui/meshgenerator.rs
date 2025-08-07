use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};

use crate::simulation::shape::{Polyhedron, Face};
use std::collections::HashSet;

fn get_face_color(_face: &Face, idx: usize, colors: &Vec<[f32; 4]>) -> [f32;4] {
	return colors[idx*4 % colors.len()];
}

fn mesh_context_from_polyherdron(p: &Polyhedron, colors: &Vec<[f32; 4]>) ->
	(
		Vec<Vec3>,
		Vec<Vec3>,
		Vec<[f32; 4]>,
		Vec<u32>,
	)
{
	let mut v_position: Vec<Vec3> = Vec::new();
	let mut v_normals: Vec<Vec3> = Vec::new();
	let mut v_colors: Vec<[f32; 4]> = Vec::new();
	let mut trigs: Vec<u32> = Vec::new();
	
	let vertex_set: &mut HashSet<usize> = &mut HashSet::new();
	for (idx,f) in p.faces.iter().enumerate() {
		let facedata = &f.get_face();
		for v in &facedata.vertices {	
			vertex_set.insert(v.index);
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
	
		v_colors.append(&mut vec![get_face_color(facedata, idx, &colors); v_count]);
	}

	return (v_position, v_normals, v_colors, trigs);
}

pub fn generate_mesh(p: &Polyhedron, colors: &Vec<[f32; 4]>) -> Mesh {
	let (v_position, v_normals, v_colors, trigs) = mesh_context_from_polyherdron(p, colors);

	Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, v_position)
	.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, v_normals)
	.with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, v_colors)
	.with_inserted_indices(Indices::U32(trigs))
}