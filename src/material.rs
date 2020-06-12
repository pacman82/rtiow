use crate::vec3::{dot, Color, Vec3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

mod diffuse;
mod metal;
pub use diffuse::{Hemisphere, Lambertian, Simple};
pub use metal::Metal;

pub trait Material {
    fn scatter(&self, rng: &mut ThreadRng, incoming: &Vec3, normal: &Vec3)
        -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub direction: Vec3,
}

fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let candidate = Vec3::random(rng, -1., 1.);
        if candidate.length_squared() < 1. {
            break candidate;
        }
    }
}

// Lambertion diffusion
fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
    let a = rng.gen_range(0., 2. * PI);
    let z: f64 = rng.gen_range(-1., 1.);
    let r = (1. - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
    *incoming - *normal * dot(*incoming, *normal) * 2.
}
