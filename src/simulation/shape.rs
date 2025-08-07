use bevy::prelude::Vec3;
use std::rc::Rc;

pub trait Facelike {
    pub fn get_face(&self) -> Face;
}

#[derive(Clone)]
pub struct Vertex {
    pub index: usize,
    pub position: Vec3,
    pub edges: Vec<Rc<Edge>>
}

#[derive(Clone)]
pub struct Edge {
    pub index: usize,
    pub vertices: [Rc<Vertex>; 2],
    pub faces: [Rc<dyn Facelike>; 2]
}

#[derive(Clone)]
pub struct Face {
    pub index: usize,
    pub edges: Vec<Rc<Edge>>,
    pub vertices: Vec<Rc<Vertex>>,
}

#[derive(Clone)]

pub struct Polyhedron {
    pub faces: Vec<Rc<dyn Facelike>>,
    pub edges: Vec<Rc<Edge>>,
    pub vertices: Vec<Rc<Vertex>>
}

impl Polyhedron {
    fn divide_edge(mut self, edgeidx: usize, weights: &[u32]) -> Polyhedron {
        let mut said_edge = Rc::clone(&self.edges[edgeidx]).as_ref();
        let mut v_line: Vec<Rc<Vertex>> = said_edge.vertices.clone().to_vec();
        let denominator: u32 = weights.iter().sum();
        let mut numerator = 0u32;

        let origin = v_line[0].as_ref().position;
        let edge_delta = origin - v_line[1].as_ref().position;

        for w in &weights[..weights.len() - 1] {
            numerator += w;
            let v = Rc::new(Vertex {
                index: self.vertices.len(),
                position: origin + edge_delta * (numerator/denominator) as f32,
                edges: Vec::new()
            });
            

            v_line.insert(v_line.len() - 2, Rc::clone(&v));

            self.vertices.push(v);
        }

        said_edge.vertices[1] = Rc::clone(&v_line[1]);
        for v_idx in 1..v_line.len() {

        }

        self
    }

    fn stitch(mut self, e1: usize, e2: usize) -> Polyhedron {

        self
    }
}