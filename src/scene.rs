use crate::{
    camera::Camera,
    ray::Ray,
    renderable::{HitCheck, Renderable},
    vec3::{Color, Vec3},
};
use rand::{rngs::ThreadRng, Rng};

pub struct Scene {
    pub world: Box<dyn Renderable + Sync + Send>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(world: Box<dyn Renderable + Sync + Send>, camera: Camera) -> Self {
        Self { world, camera }
    }

    pub fn render_sample(
        &self,
        rng: &mut ThreadRng,
        max_depth: u32,
        image_height: u32,
        image_width: u32,
    ) -> Vec<Color> {
        let pixels = (0..image_height)
            .rev()
            .flat_map(|j| (0..image_width).map(move |i| (i, j)));
        pixels
            .map(|(i, j)| {
                let u = (i as f64 + rng.gen_range(0., 1.)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0., 1.)) / (image_height - 1) as f64;
                let ray = self.camera.get_ray(u, v, rng);
                let time = self.camera.get_time(rng);
                ray_color(ray, time, self.world.as_ref(), rng, max_depth)
            })
            .collect()
    }
}

fn ray_color(
    mut ray: Ray,
    time: f64,
    world: &dyn Renderable,
    rng: &mut ThreadRng,
    depth: u32,
) -> Color {
    let mut trace = |ray| {
        match world.hit_check(&ray, 0.001, f64::INFINITY, time, rng) {
            // No object in the scene has been hit. Let's use the ambient light.
            HitCheck::Miss => (ambient(&ray.direction), None),
            HitCheck::Absorbed => (Color::new(0., 0., 0.), None),
            HitCheck::Reflected {
                attenuation,
                scattered,
            } => (attenuation, Some(scattered)),
        }
    };

    let mut color = Color::new(1., 1., 1.);
    for _ in 0..depth {
        let (attenuation, scattered) = trace(ray);
        color *= attenuation;
        if let Some(next_ray) = scattered {
            ray = next_ray
        } else {
            break;
        }
    }
    // In case of us hitting the max depth limit the original code proposed by the rtiow to return
    // black, I find it however more appealing to return the attenuated color so far. For
    // increasingly higher depth limits both variants become similar anyway.
    color
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
