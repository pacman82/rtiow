use crate::{
    persistence::{HittableBuilder, MaterialBuilder, SceneBuilder, ShapeBuilder, CameraBuilder},
    vec3::{Color, Point, Vec3},
};
use rand::Rng;

pub fn spheres(rng: &mut impl Rng, aspect_ratio: f64) -> SceneBuilder {
    let mut world = Vec::new();
    let ground_material = MaterialBuilder::Diffuse {
        albedo: Color::new(0.5, 0.5, 0.5),
    };

    world.push(HittableBuilder {
        shape: ShapeBuilder::Sphere {
            center: Point::new(0., -1000., 0.),
            radius: 1000.,
        },
        material: ground_material,
        velocity: None,
    });

    let small_radius = 0.2;

    for a in (-11..11).map(|i| i as f64) {
        for b in (-11..11).map(|i| i as f64) {
            let center = Point::new(
                a + rng.gen_range(0., 0.9),
                small_radius,
                b + rng.gen_range(0., 0.9),
            );

            if (center - Point::new(4., small_radius, 0.)).length() > 0.9 {
                let (material, velocity) = if rng.gen_bool(0.8) {
                    let albedo = &Color::random(rng, 0., 1.) * &Color::random(rng, 0., 1.);
                    (
                        MaterialBuilder::Diffuse { albedo },
                        Some(Vec3::new(0., rng.gen_range(0., 0.5), 0.)),
                    )
                } else if rng.gen_bool(0.75) {
                    let albedo = Color::random(rng, 0.5, 1.);
                    let fuzziness = rng.gen_range(0., 0.5);
                    (MaterialBuilder::Metal { albedo, fuzziness }, None)
                } else {
                    (
                        MaterialBuilder::Dielectric {
                            refractive_index: 1.5,
                        },
                        None,
                    )
                };
                let little_ball = HittableBuilder {
                    shape: ShapeBuilder::Sphere {
                        center,
                        radius: small_radius,
                    },
                    material,
                    velocity,
                };
                world.push(little_ball);
            }
        }
    }

    world.push(HittableBuilder {
        shape: ShapeBuilder::Sphere {
            center: Point::new(0., 1., 0.),
            radius: 1.0,
        },
        material: MaterialBuilder::Dielectric {
            refractive_index: 1.5,
        },
        velocity: None,
    });

    world.push(HittableBuilder {
        shape: ShapeBuilder::Sphere {
            center: Point::new(-4., 1., 0.),
            radius: 1.0,
        },
        material: MaterialBuilder::Diffuse {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
        velocity: None,
    });

    world.push(HittableBuilder {
        shape: ShapeBuilder::Sphere {
            center: Point::new(4., 1., 0.),
            radius: 1.0,
        },
        material: MaterialBuilder::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzziness: 0.,
        },
        velocity: None,
    });

    let camera = CameraBuilder {
        vertical_field_of_view: 20.,
        aspect_ratio,
        lookfrom: Point::new(13., 2., 3.),
        lookat: Point::new(0., 0., 0.),
        view_up: Vec3::new(0., 1., 0.),
        distance_to_focus: 10.,
        aperture: 0.1,
        exposure_time: 1.,
    };

    SceneBuilder { camera, world }
}
