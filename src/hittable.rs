use crate::{
    ray::Ray,
    vec3::{dot, Point, Vec3},
};

pub struct HitRecord {
    pub t: f64,
    pub point: Point,
    pub front_face: bool,
    /// Always pointing against the intersecting ray.
    pub normal: Vec3,
}

impl HitRecord {
    pub fn from_outward_normal(t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let point = ray.at(t);
        let front_face = dot(outward_normal, ray.direction) < 0.;
        Self {
            t,
            point,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
