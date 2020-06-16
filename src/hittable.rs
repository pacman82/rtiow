use crate::{
    material::Material,
    ray::Ray,
    shape::{Intersection, Shape},
};
use std::ops::Deref;

pub struct HitRecord<'m> {
    pub intersection: Intersection,
    pub material: &'m dyn Material,
}

impl<'m> HitRecord<'m> {
    pub fn new(intersection: Intersection, material: &'m dyn Material) -> Self {
        Self {
            intersection,
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
            let closest_so_far = rec.as_ref().map(|r| r.intersection.t).unwrap_or(t_max);
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

pub struct ShapeWithMaterial<S, M> {
    shape: S,
    material: M,
}

impl<S, M> ShapeWithMaterial<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        ShapeWithMaterial { shape, material }
    }
}

impl<S, M> Hittable for ShapeWithMaterial<S, M>
where
    S: Shape,
    M: Material,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.shape
            .intersect(ray, t_min, t_max)
            .map(|intersection| HitRecord::new(intersection, &self.material))
    }
}
