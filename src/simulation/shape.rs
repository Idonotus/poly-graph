use bevy::prelude::Vec3;

pub struct Vertex {
    pub position: Vec3,
    pub edges: Vec<usize>
}

pub struct Edge {
    pub vertices: [usize; 2],
    pub faces: [usize; 2]
}

pub struct Face {
    pub edges: Vec<usize>,
    pub vertices: Vec<usize>,
    pub positon: Vec3
}

pub struct Polyhedron {
    pub faces: Vec<Face>,
    pub edges: Vec<Edge>,
    pub vertices: Vec<Vertex>
}