// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
mod hittable;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use std::io;
use vec3::{Color, Point, Vec3};

fn main() -> io::Result<()> {
    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // P3 means colors are in ASCII. Then width, then height. 255 is the max color.
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    let world = vec![
        Sphere::new(Point::new(0., 0., -1.), 0.5),
        Sphere::new(Point::new(0., -100.5, -1.), 100.),
    ];

    for j in (0..image_height).into_iter().rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::from_to(origin, lower_left_corner + horizontal * u + vertical * v);
            let pixel_color = ray_color(&ray, &world);
            write_color(&pixel_color, io::stdout().lock())?;
        }
    }
    eprintln!("Done!");
    Ok(())
}

pub fn write_color(color: &Color, mut out: impl io::Write) -> io::Result<()> {
    let red = (255.999 * color[0]) as i32;
    let green = (255.999 * color[1]) as i32;
    let blue = (255.999 * color[2]) as i32;

    // Now print RGB tripplets
    writeln!(out, "{} {} {}", red, green, blue)
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(ray, 0., 1.) {
        (rec.normal + Color::new(1., 1., 1.)) / 2.
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
