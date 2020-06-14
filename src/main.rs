// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// Online ppm viewer: http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
mod camera;
mod hittable;
mod material;
mod output;
mod ray;
mod sphere;
mod vec3;
mod world;

use crate::{
    camera::Camera,
    hittable::Hittable,
    output::save_image,
    ray::Ray,
    vec3::{Color, Point, Vec3},
    world::create_world,
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;
use std::{io, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, default_value = "100")]
    samples_per_pixel: u32,
    #[structopt(long, default_value = "50")]
    max_depth: u32,
    #[structopt(long, default_value = "384")]
    image_width: u32,
    #[structopt(long, short = "o", default_value = "image.png")]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let Cli {
        samples_per_pixel,
        max_depth,
        image_width,
        output,
    } = Cli::from_args();

    let aspect_ratio = 16. / 9.;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let mut rng = thread_rng();

    let world = create_world(&mut rng);

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

    let progress_bar = ProgressBar::new(image_height as u64 * image_width as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar().template("[{elapsed_precise}][{eta}] {bar:80.cyan/blue}  "),
    );

    let _pixels = (0..image_height)
        .rev()
        .flat_map(|j| (0..image_width).map(move |i| (i, j)));

    // Probably better to parallise sampling, but let's just hack rayon in there real quick.
    let mut scanlines = Vec::new();
    (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            let mut rng = thread_rng();
            let scanline = (0..image_width)
                .map(|i| {
                    let acc_color = (0..samples_per_pixel)
                        .map(|_| {
                            let u = (i as f64 + rng.gen_range(0., 1.)) / (image_width - 1) as f64;
                            let v = (j as f64 + rng.gen_range(0., 1.)) / (image_height - 1) as f64;
                            let ray = camera.get_ray(u, v, &mut rng);
                            ray_color(&ray, &world, &mut rng, max_depth)
                        })
                        .fold(Color::new(0., 0., 0.), |a, b| a + b);
                    let pixel_color = acc_color / samples_per_pixel as f64;
                    progress_bar.inc(1);
                    pixel_color
                })
                .collect::<Vec<_>>();
            scanline
        })
        .collect_into_vec(&mut scanlines);

    progress_bar.finish();

    save_image(&scanlines, &output)?;

    eprintln!("Done!");

    Ok(())
}

fn ray_color(ray: &Ray, world: &dyn Hittable, rng: &mut ThreadRng, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scattered) =
            rec.material
                .scatter(rng, &ray.direction, &rec.normal, rec.front_face)
        {
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
