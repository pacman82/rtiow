// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
mod ray;
mod vec3;

use ray::Ray;
use std::io;
use vec3::{Color, Point, Vec3, dot};

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

    for j in (0..image_height).into_iter().rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::from_to(origin, lower_left_corner + horizontal * u + vertical * v);
            let pixel_color = ray_color(&ray);
            pixel_color.write_color(io::stdout().lock())?;
        }
    }
    eprintln!("Done!");
    Ok(())
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point::new(0., 0., -1.), 0.5, &ray) {
        Color::new(1., 0., 0.)
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

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> bool {
    let mut ray = *ray;
    // Transform coordinats so sphere is in the center.
    ray.origin -= *center;
    // r^2 == (t * D + O) * (t * D + O)
    // r^2 == t^2 D*D + 2t*D*O + O*O
    // 0 == t^2 D*D + 2t*D*O + O*O - r^2
    let a = ray.direction.length_squared();
    let b = 2. * dot(ray.direction, ray.origin);
    let c = ray.origin.length_squared() - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.
}
