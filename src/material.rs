use crate::vec3::{dot, Color, Vec3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

mod dielectric;
mod diffuse;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse::{Hemisphere, Lambertian, Simple};
pub use metal::Metal;

pub trait Material {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        incoming: &Vec3,
        normal: &Vec3,
        front_face: bool,
    ) -> Option<ScatterResult>;
}

impl<M> Material for Box<M>
where
    M: Material + ?Sized,
{
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        incoming: &Vec3,
        normal: &Vec3,
        front_face: bool,
    ) -> Option<ScatterResult> {
        self.as_ref().scatter(rng, incoming, normal, front_face)
    }
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

fn refract(unit_incoming: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-*unit_incoming, *normal);
    let r_out_parallel = (*unit_incoming + *normal * cos_theta) * etai_over_etat;
    let r_out_perpendiclar = -*normal * (1. - r_out_parallel.length_squared()).sqrt();
    r_out_parallel + r_out_perpendiclar
}

/// Approximate probability to reflect.
fn schlick(cosine: f64, refracture_index: f64) -> f64 {
    let r0 = (1. - refracture_index) / (1. + refracture_index);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
