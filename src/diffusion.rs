use crate::vec3::{dot, Vec3};
use rand::Rng;
use std::f64::consts::PI;


pub trait Diffusion<R: Rng> {
    fn diffuse(&self, rng: &mut R, normal: &Vec3) -> Vec3;
}

pub struct Lambertian;

impl<R: Rng> Diffusion<R> for Lambertian {
    fn diffuse(&self, rng: &mut R, normal: &Vec3) -> Vec3 {
        *normal + random_unit_vector(rng)    
    }
}

pub struct Simple;

impl<R: Rng> Diffusion<R> for Simple {
    fn diffuse(&self, rng: &mut R, normal: &Vec3) -> Vec3 {
        *normal + random_in_unit_sphere(rng)    
    }
}

pub struct Hemisphere;

/// A more intuitive approach is to have a uniform scatter direction for all angles away from the
/// hit point, with no dependence on the angle from the normal. Many of the first raytracing papers
/// used this diffuse method (before adopting Lambertian diffuse). 
impl<R: Rng> Diffusion<R> for Hemisphere {
    fn diffuse(&self, rng: &mut R, normal: &Vec3) -> Vec3 {
        let in_unit_sphere = random_in_unit_sphere(rng);
        if dot(in_unit_sphere, *normal) > 0.0 { // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
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
