// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::{
    camera::Camera,
    hittable::Hittable,
    ray::Ray,
    sphere::Sphere,
    vec3::{Color, Point},
};
use rand::{thread_rng, Rng};
use std::io;

fn main() -> io::Result<()> {
    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // P3 means colors are in ASCII. Then width, then height. 255 is the max color.
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let world = vec![
        Sphere::new(Point::new(0., 0., -1.), 0.5),
        Sphere::new(Point::new(0., -100.5, -1.), 100.),
    ];

    let camera = Camera::new(aspect_ratio);

    let mut rng = thread_rng();

    for j in (0..image_height).into_iter().rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let acc_color = (0..samples_per_pixel)
                .into_iter()
                .map(|_| {
                    let u = (i as f64 + rng.gen_range(0., 1.)) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0., 1.)) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &world, &mut rng, max_depth)
                })
                .fold(Color::new(0., 0., 0.), |a, b| a + b);
            let pixel_color = acc_color / samples_per_pixel as f64;
            write_color(&pixel_color, io::stdout().lock())?;
        }
    }
    eprintln!("Done!");
    Ok(())
}

pub fn write_color(color: &Color, mut out: impl io::Write) -> io::Result<()> {
    let red = (256. * clamp(color[0], 0., 0.999)) as i32;
    let green = (256. * clamp(color[1], 0., 0.999)) as i32;
    let blue = (256. * clamp(color[2], 0., 0.999)) as i32;

    // Now print RGB tripplets
    writeln!(out, "{} {} {}", red, green, blue)
}

fn ray_color(ray: &Ray, world: &impl Hittable, rng: &mut impl Rng, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(ray, 0., f64::INFINITY) {
        let target = rec.point + rec.normal + random_in_unit_sphere(rng);
        let ray = Ray::from_to(rec.point, target);
        ray_color(&ray, world, rng, depth - 1) * 0.5
    } else {
        // y is between -1 and 1
        let y = ray.direction.unit().y();
        // 0 <= t <= 1
        let t = (y + 1.) / 2.;
        let blend_start = Color::new(1., 1., 1.);
        let blend_end = Color::new(0.5, 0.7, 1.0);
        blend_start * (1. - t) + blend_end * t
    }
}

fn clamp(f: f64, low: f64, high: f64) -> f64 {
    if f < low {
        low
    } else if f > high {
        high
    } else {
        f
    }
}

fn random_in_unit_sphere(rng: &mut impl Rng) -> Point {
    loop {
        let candidate = Point::random(rng, -1., 1.);
        if candidate.length_squared() < 1. {
            break candidate;
        }
    }
}
