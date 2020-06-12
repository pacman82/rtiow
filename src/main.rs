// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

use crate::{
    camera::Camera,
    hittable::Hittable,
    ray::Ray,
    vec3::{Color, Vec3},
    world::create_world,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, default_value = "100")]
    samples_per_pixel: u32,
    #[structopt(long, default_value = "50")]
    max_depth: u32,
}

fn main() -> io::Result<()> {
    let Cli {
        samples_per_pixel,
        max_depth,
    } = Cli::from_args();

    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // P3 means colors are in ASCII. Then width, then height. 255 is the max color.
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let world = create_world();

    let camera = Camera::new(aspect_ratio);

    let mut rng = thread_rng();

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let acc_color = (0..samples_per_pixel)
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
    let red = (256. * clamp(color[0].sqrt(), 0., 0.999)) as i32;
    let green = (256. * clamp(color[1].sqrt(), 0., 0.999)) as i32;
    let blue = (256. * clamp(color[2].sqrt(), 0., 0.999)) as i32;

    // Now print RGB tripplets
    writeln!(out, "{} {} {}", red, green, blue)
}

fn ray_color(ray: &Ray, world: &dyn Hittable, rng: &mut ThreadRng, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scattered) = rec.material.scatter(rng, &ray.direction, &rec.normal) {
            let target = rec.point + scattered.direction;
            let ray = Ray::from_to(rec.point, target);
            &ray_color(&ray, world, rng, depth - 1) * &scattered.attenuation
        } else {
            Color::new(0., 0., 0.)
        }
    } else {
        ambient(&ray.direction)
    }
}

fn ambient(direction: &Vec3) -> Color {
    // y is between -1 and 1
    let y = direction.unit().y();
    // 0 <= t <= 1
    let t = (y + 1.) / 2.;
    let blend_start = Color::new(1., 1., 1.);
    let blend_end = Color::new(0.5, 0.7, 1.0);
    blend_start * (1. - t) + blend_end * t
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
