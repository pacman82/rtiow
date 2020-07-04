use crate::{hittable::Hittable, ray::Ray, vec3::Color};
use rand::rngs::ThreadRng;

pub trait Renderable {
    fn hit_check(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        time: f64,
        rng: &mut ThreadRng,
    ) -> HitCheck;
}

/// Possible interactions of a ray with objects in the scene.
pub enum HitCheck {
    /// The ray did not hit the object
    Miss,
    /// The ray hid the object and has been completly absorbed by it.
    Absorbed,
    /// The ray hit the object and has been scattered by its surface.
    Reflected { attenuation: Color, scattered: Ray },
}

impl<R> Renderable for R
where
    R: Hittable + ?Sized,
{
    fn hit_check(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        time: f64,
        rng: &mut ThreadRng,
    ) -> HitCheck {
        if let Some(hit) = self.hit(ray, t_min, t_max, time) {
            if let Some(scattered) = hit.material.scatter(
                rng,
                &ray.direction,
                &hit.intersection.normal,
                hit.intersection.front_face,
            ) {
                HitCheck::Reflected {
                    attenuation: scattered.attenuation,
                    scattered: Ray::new(hit.intersection.point, scattered.direction),
                }
            } else {
                HitCheck::Absorbed
            }
        } else {
            HitCheck::Miss
        }
    }
}
