use crate::{
    material::Material,
    ray::Ray,
    vec3::{dot, Point, Vec3},
};
use std::ops::Deref;

pub struct HitRecord<'m> {
    pub t: f64,
    pub point: Point,
    pub front_face: bool,
    /// Always pointing against the intersecting ray.
    pub normal: Vec3,
    pub material: &'m dyn Material,
}

impl<'m> HitRecord<'m> {
    pub fn from_outward_normal(
        t: f64,
        ray: &Ray,
        outward_normal: Vec3,
        material: &'m dyn Material,
    ) -> Self {
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
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter().fold(None, |rec, hittable| {
            let closest_so_far = rec.as_ref().map(|r| r.t).unwrap_or(t_max);
            if let Some(rec) = hittable.hit(ray, t_min, closest_so_far) {
                Some(rec)
            } else {
                rec
            }
        })
    }
}

impl<T> Hittable for Box<T>
where
    T: Hittable + ?Sized,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.deref().hit(ray, t_min, t_max)
    }
}
