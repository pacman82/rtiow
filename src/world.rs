use crate::{
    hittable::Hittable,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point},
};

pub fn create_world() -> Vec<Box<dyn Hittable>> {
    vec![
        Box::new(Sphere::new(
            Point::new(0., 0., -1.),
            0.5,
            Lambertian::new(Color::new(0.1, 0.2, 0.5)),
        )),
        Box::new(Sphere::new(
            Point::new(0., -100.5, -1.),
            100.,
            Lambertian::new(Color::new(0.8, 0.8, 0.0)),
        )),
        Box::new(Sphere::new(
            Point::new(1., 0., -1.),
            0.5,
            Metal::new(Color::new(0.8, 0.6, 0.2), 0.0),
        )),
        Box::new(Sphere::new(
            Point::new(-1., 0., -1.),
            0.5,
            Dielectric::new(1.5),
        )),
    ]
}