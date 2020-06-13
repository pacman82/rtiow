use crate::{
    ray::Ray,
    vec3::{cross, Point, Vec3},
};
use rand::Rng;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        lookfrom: Point,
        lookat: Point,
        view_up: Vec3,
        distance_to_focus: f64,
        aperture: f64,
    ) -> Self {
        let theta = vertical_field_of_view.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        // Distance between projection point and image plane.
        let focal_length = 1.;

        let w = (lookfrom - lookat).unit() * focal_length;
        let u = cross(&view_up, &w).unit();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * distance_to_focus;
        let vertical = v * viewport_height * distance_to_focus;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w * distance_to_focus;

        let lens_radius = aperture / 2.;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut impl Rng) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::from_to(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t,
        )
    }
}

fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    loop {
        let x = rng.gen_range(-1., 1.);
        let y: f64 = rng.gen_range(-1., 1.);
        if x * x + y * y < 1. {
            break Vec3::new(x, y, 0.);
        }
    }
}
