use super::{reflect, Material, ScatterResult};
use crate::vec3::{Color, Vec3, dot};
use rand::rngs::ThreadRng;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        _rng: &mut ThreadRng,
        incoming: &Vec3,
        normal: &Vec3,
    ) -> Option<ScatterResult> {
        // Why do we need the unit vector? Or do we?
        let scattered = reflect(&incoming.unit(), normal);
        if dot(scattered, *normal) > 0. {
            Some(ScatterResult {
                attenuation: self.albedo,
                direction: scattered,
            })
        } else {
            // Does this actually happen?
            None
        }
    }
}
