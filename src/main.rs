use std::{ops::{Add, Mul}, rc::Rc};


struct Point { x: f64, y: f64, z: f64 }

impl Mul<Point> for Point {
    type Output = f64;

    fn mul(self, rhs: Point) -> Self::Output {
        return self.x*rhs.x + self.y*rhs.y + self.z*rhs.z;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        return Point{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z};
    }
}

struct Vertex {
    position: Point,
    edges: Vec<Rc<Edge>>
}

struct Edge {
    vertices: [Rc<Vertex>; 2],
    faces: [Rc<Face>; 2]
}

struct Face {
    edges: Vec<Rc<Edge>>,
    positon: Point
}

fn main() {
    println!("Hello, world!");
}
