use bevy::prelude::Vec3;
use std::rc::Rc;

pub struct Vertex {
    position: Vec3,
    edges: Vec<Rc<Edge>>
}

pub struct Edge {
    vertices: [Rc<Vertex>; 2],
    faces: [Rc<Face>; 2]
}

pub struct Face {
    edges: Vec<Rc<Edge>>,
    positon: Vec3
}