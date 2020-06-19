use crate::{camera::Camera, hittable::Hittable};

pub struct Scene {
    pub world: Vec<Box<dyn Hittable + Sync + Send>>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(world: Vec<Box<dyn Hittable + Sync + Send>>, camera: Camera) -> Self {
        Self { world, camera }
    }
}
