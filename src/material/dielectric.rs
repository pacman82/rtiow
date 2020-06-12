// Eta air = 1.0 glass = 1.3 1.7 diamond = 1.4

use super::{refract, Material, ScatterResult, reflect};
use crate::vec3::{Color, Vec3, dot};
use rand::rngs::ThreadRng;

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self {
            refractive_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        _rng: &mut ThreadRng,
        incoming: &Vec3,
        normal: &Vec3,
        front_face: bool,
    ) -> Option<ScatterResult> {
        let unit_incoming = incoming.unit();
        let cos_theta = dot(-unit_incoming, *normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let etai_over_etat = if front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };
        let direction = if etai_over_etat * sin_theta > 1.0 {
            // Must Reflect
            reflect(&unit_incoming, normal)
        }
        else {
            // Can Refract
            refract(&unit_incoming, normal, etai_over_etat)
        };

        Some(ScatterResult {
            direction,
            attenuation: Color::new(1., 1., 1.),
        })
    }
}