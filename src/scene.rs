use crate::{camera::Camera, hittable::Hittable};

pub struct Scene {
    pub world: Box<dyn Hittable + Sync + Send>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(world: Box<dyn Hittable + Sync + Send>, camera: Camera) -> Self {
        Self { world, camera }
    }
}
