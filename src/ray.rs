pub use crate::vec3::{Point, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn from_to(origin: Point, destination: Point) -> Self {
        Self::new(origin, destination - origin)
    }
}
