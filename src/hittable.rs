use crate::{
    material::Material,
    ray::Ray,
    shape::{Intersection, Shape},
};

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
        self.as_ref().hit(ray, t_min, t_max)
    }
}

impl<S, M> Hittable for (S,M)
where
    S: Shape,
    M: Material,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.0
            .intersect(ray, t_min, t_max)
            .map(|intersection| HitRecord::new(intersection, &self.1))
    }
}
