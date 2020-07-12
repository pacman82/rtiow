use crate::{
    bounding_box::{Aabb, BoundingBox},
    hittable::{Hit, Hittable},
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, time: f64) -> Option<(f64, Hit)> {
        let mut ray_in_object_coordinates = *ray;
        ray_in_object_coordinates.origin -= self.velocity * time;
        self.inner
            .hit(&ray_in_object_coordinates, t_min, t_max, time)
            .map(|(distance, mut hit_record)| {
                hit_record.intersection.point += self.velocity * time;
                (distance, hit_record)
            })
    }
}

impl<B> BoundingBox for Moving<B>
where
    B: BoundingBox,
{
    fn bounding_box(&self, exposure_time: f64) -> Aabb {
        let box_t_min = self.inner.bounding_box(exposure_time);
        let box_t_max = box_t_min.shifted(&self.velocity);
        Aabb::surrounding(&box_t_min, &box_t_max)
    }
}
