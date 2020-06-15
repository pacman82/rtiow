use crate::{
    camera::Camera,
    hittable::Hittable,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point, Vec3},
};
use rand::Rng;

pub struct Scene {
    pub world: Vec<Box<dyn Hittable + Sync + Send>>,
    pub camera: Camera,
}

impl Scene {
    pub fn mostly_random_spheres(rng: &mut impl Rng, aspect_ratio: f64) -> Self {
        let lookfrom = Point::new(13., 2., 3.);
        let lookat = Point::new(0., 0., 0.);
        let distance_to_focus = 10.;

        let camera = Camera::new(
            20.,
            aspect_ratio,
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            distance_to_focus,
            0.1,
        );

        let world = create_world_with_random_spheres(rng);

        Scene { world, camera }
    }
}

fn create_world_with_random_spheres(rng: &mut impl Rng) -> Vec<Box<dyn Hittable + Sync + Send>> {
    let mut world: Vec<Box<dyn Hittable + Sync + Send>> = Vec::new();
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    world.push(Box::new(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));
    for a in (-11..11).map(|i| i as f64) {
        for b in (-11..11).map(|i| i as f64) {
            let center = Point::new(
                a + 0.9 * rng.gen_range(0., 1.),
                0.2,
                b + 0.9 * rng.gen_range(0., 1.),
            );
            if (center - Point::new(4., 0.2, 0.)).length() > 0.9 {
                if rng.gen_bool(0.8) {
                    let albedo = &Color::random(rng, 0., 1.) * &Color::random(rng, 0., 1.);
                    let material = Lambertian::new(albedo);
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if rng.gen_bool(0.75) {
                    let albedo = Color::random(rng, 0.5, 1.);
                    let fuzz = rng.gen_range(0., 0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point::new(0., 1., 0.),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Point::new(-4., 1., 0.),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point::new(4., 1., 0.),
        1.0,
        material3,
    )));
    world
}
