use crate::{
    camera::Camera,
    hittable::Hittable,
    ray::Ray,
    vec3::{Color, Vec3},
};
use rand::{rngs::ThreadRng, Rng};

pub fn render_sample(
    rng: &mut ThreadRng,
    max_depth: u32,
    image_height: u32,
    image_width: u32,
    camera: &Camera,
    world: &dyn Hittable,
) -> Vec<Color> {
    let pixels = (0..image_height)
        .rev()
        .flat_map(|j| (0..image_width).map(move |i| (i, j)));
    pixels
        .map(|(i, j)| {
            let u = (i as f64 + rng.gen_range(0., 1.)) / (image_width - 1) as f64;
            let v = (j as f64 + rng.gen_range(0., 1.)) / (image_height - 1) as f64;
            let ray = camera.get_ray(u, v, rng);
            ray_color(&ray, world, rng, max_depth)
        })
        .collect()
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
