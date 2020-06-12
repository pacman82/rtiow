use super::{random_in_unit_sphere, reflect, Material, ScatterResult};
use crate::vec3::{dot, Color, Vec3};
use rand::rngs::ThreadRng;

pub struct Metal {
    albedo: Color,
    // Should be a value from 0. to 1.
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        incoming: &Vec3,
        normal: &Vec3,
        _front_face: bool,
    ) -> Option<ScatterResult> {
        let reflected = reflect(incoming, normal);
        if dot(reflected, *normal) > 0. {
            Some(ScatterResult {
                attenuation: self.albedo,
                direction: reflected.unit() + random_in_unit_sphere(rng) * self.fuzz,
            })
        } else {
            // This should only happen for fuzz > 0. Big rays or grazing rays, may scatter below the
            // surface.
            None
        }
    }
}
