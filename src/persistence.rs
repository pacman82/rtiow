use crate::{
    bvh::{into_bounding_volume_hierarchy, BoundedHittable},
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    moving::Moving,
    scene::Scene,
    shape::Sphere,
    vec3::{Color, Point, Vec3},
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, io, path::Path};

/// Serializable representation of a Scene. Used to persist scenes to '.toml' files.
#[derive(Serialize, Deserialize)]
pub struct SceneBuilder {
    camera: CameraBuilder,
    world: Vec<HittableBuilder>,
}

impl SceneBuilder {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let text = read_to_string(path)?;
        let desc = serde_json::from_str(&text)?;
        Ok(desc)
    }

    pub fn to_path(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let text = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(&path, text)
    }

    pub fn mostly_random_spheres(rng: &mut impl Rng, aspect_ratio: f64) -> Self {
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

        Self { camera, world }
    }

    pub fn build(&self) -> Scene {
        let hittables = self.world.iter().map(|model| model.build()).collect();
        let world = into_bounding_volume_hierarchy(hittables, self.camera.exposure_time);
        // let hittables: Vec<_> = self.world.iter().map(|model| model.build()).collect();
        // let world = Box::new(hittables);
        let camera = self.camera.build();

        Scene::new(world, camera)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CameraBuilder {
    vertical_field_of_view: f64,
    /// Width divided by height of the image to render.
    aspect_ratio: f64,
    /// Position of the camera.
    lookfrom: Point,
    /// Point the camera is looking at. Used to determine the direction of the camera.
    lookat: Point,
    view_up: Vec3,
    distance_to_focus: f64,
    aperture: f64,
    /// Use this for motion blur. Rays will be emitted randomly between t0=0 and t1=exposure_time.
    /// Can also be understood as the time it takes for the shutter to close.
    exposure_time: f64,
}

impl CameraBuilder {
    fn build(&self) -> Camera {
        Camera::new(
            self.vertical_field_of_view,
            self.aspect_ratio,
            self.lookfrom,
            self.lookat,
            self.view_up,
            self.distance_to_focus,
            self.aperture,
            self.exposure_time,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MaterialBuilder {
    Diffuse { albedo: Color },
    Metal { albedo: Color, fuzziness: f64 },
    Dielectric { refractive_index: f64 },
}

impl MaterialBuilder {
    fn build(self) -> Box<dyn Material + Send + Sync> {
        match self {
            MaterialBuilder::Diffuse { albedo } => Box::new(Lambertian::new(albedo)),
            MaterialBuilder::Metal { albedo, fuzziness } => Box::new(Metal::new(albedo, fuzziness)),
            MaterialBuilder::Dielectric { refractive_index } => {
                Box::new(Dielectric::new(refractive_index))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum ShapeBuilder {
    Sphere { center: Point, radius: f64 },
}

impl ShapeBuilder {
    fn build(self) -> Sphere {
        match self {
            ShapeBuilder::Sphere { center, radius } => Sphere::new(center, radius),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct HittableBuilder {
    shape: ShapeBuilder,
    material: MaterialBuilder,
    velocity: Option<Vec3>,
}

impl HittableBuilder {
    fn build(&self) -> Box<dyn BoundedHittable> {
        if let Some(velocity) = self.velocity {
            Box::new(Moving::new(
                velocity,
                (self.shape.build(), self.material.build()),
            ))
        } else {
            Box::new((self.shape.build(), self.material.build()))
        }
    }
}
