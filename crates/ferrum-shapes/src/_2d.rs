use ferrum_types::SimpleMesh;


pub struct Circle {
    pub mesh: SimpleMesh
}

impl Circle {
    pub fn new(x: f32, y: f32) {

    }
}

pub struct Rectangle {
    pub mesh: SimpleMesh
}

impl Rectangle {
    pub fn new(x: f32, y: f32) {

    }
}

pub struct Triangle {
    pub mesh: SimpleMesh
}

impl Triangle {

    pub fn simple(x: f32, y: f32) {
        Self::new(x, y, 0.1, 0.1, 0.1);
    }

    pub fn new(x: f32, y: f32, a: f32, b: f32, c: f32) {

    }
}