use super::{random_in_unit_sphere, random_unit_vector, Material, ScatterResult};
use crate::vec3::{dot, Color, Vec3};
use rand::rngs::ThreadRng;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        _incoming: &Vec3,
        normal: &Vec3,
        _front_face: bool,
    ) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.albedo,
            direction: *normal + random_unit_vector(rng),
        })
    }
}

pub struct Simple {
    pub albedo: Color,
}

impl Material for Simple {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        _incoming: &Vec3,
        normal: &Vec3,
        _front_face: bool,
    ) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.albedo,
            direction: *normal + random_in_unit_sphere(rng),
        })
    }
}

pub struct Hemisphere {
    pub albedo: Color,
}

/// A more intuitive approach is to have a uniform scatter direction for all angles away from the
/// hit point, with no dependence on the angle from the normal. Many of the first raytracing papers
/// used this diffuse method (before adopting Lambertian diffuse).
impl Material for Hemisphere {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        _incoming: &Vec3,
        normal: &Vec3,
        _front_face: bool,
    ) -> Option<ScatterResult> {
        let in_unit_sphere = random_in_unit_sphere(rng);
        let direction = if dot(in_unit_sphere, *normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        };
        Some(ScatterResult {
            attenuation: self.albedo,
            direction,
        })
    }
}
