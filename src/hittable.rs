use crate::{
    ray::Ray,
    shape::{Puncture, Shape},
    texture::Texture,
};

pub struct Hit<'m> {
    pub intersection: Puncture,
    pub texture: &'m dyn Texture,
}

impl<'m> Hit<'m> {
    pub fn new(intersection: Puncture, texture: &'m dyn Texture) -> Self {
        Self {
            intersection,
            texture,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, time: f64) -> Option<(f64, Hit)>;
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, time: f64) -> Option<(f64, Hit)> {
        self.iter().fold(None, |rec, hittable| {
            let closest_so_far = rec
                .as_ref()
                .map(|(distance, _hit)| *distance)
                .unwrap_or(t_max);
            if let Some(rec) = hittable.hit(ray, t_min, closest_so_far, time) {
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, time: f64) -> Option<(f64, Hit)> {
        self.as_ref().hit(ray, t_min, t_max, time)
    }
}

impl<S, T> Hittable for (S, T)
where
    S: Shape,
    T: Texture,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _time: f64) -> Option<(f64, Hit)> {
        self.0
            .intersect(ray, t_min, t_max)
            .map(|(distance, intersection)| {
                let texture = &self.1;
                (distance, Hit::new(intersection, texture))
            })
    }
}
