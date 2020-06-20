use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Moving<H> {
    velocity: Vec3,
    inner: H,
}

impl<H> Moving<H> {
    pub fn new(velocity: Vec3, inner: H) -> Self {
        Self { velocity, inner }
    }
}

impl<H> Hittable for Moving<H>
where
    H: Hittable,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, time: f64) -> Option<HitRecord> {
        let mut ray_in_object_coordinates = *ray;
        ray_in_object_coordinates.origin -= self.velocity * time;
        self.inner
            .hit(&ray_in_object_coordinates, t_min, t_max, time)
            .map(|mut hit_record| {
                hit_record.intersection.point += self.velocity * time;
                hit_record
            })
    }
}
