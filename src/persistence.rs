use crate::{
    camera::Camera,
    hittable::{Hittable, ShapeWithMaterial},
    material::{Dielectric, Lambertian, Material, Metal},
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
    lookfrom: Point,
    lookat: Point,
    distance_to_focus: f64,
    vertical_field_of_view: f64,
    view_up: Vec3,
    aspect_ratio: f64,
    aperture: f64,
    world: Vec<ShapeWithMaterialBuilder>,
}

impl SceneBuilder {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let text = read_to_string(path)?;
        let desc = serde_json::from_str(&text)?;
        Ok(desc)
    }

    pub fn to_path(&self, path: impl AsRef<Path>) -> io::Result <()> {
        let text = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(&path, text)
    }

    pub fn mostly_random_spheres(rng: &mut impl Rng, aspect_ratio: f64) -> Self {
        let mut world = Vec::new();
        let ground_material = MaterialBuilder::Diffuse {
            albedo: Color::new(0.5, 0.5, 0.5),
        };

        world.push(ShapeWithMaterialBuilder {
            shape: ShapeBuilder::Sphere {
                center: Point::new(0., -1000., 0.),
                radius: 1000.,
            },
            material: ground_material,
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
                    let material = if rng.gen_bool(0.8) {
                        let albedo = &Color::random(rng, 0., 1.) * &Color::random(rng, 0., 1.);
                        MaterialBuilder::Diffuse { albedo }
                    } else if rng.gen_bool(0.75) {
                        let albedo = Color::random(rng, 0.5, 1.);
                        let fuzziness = rng.gen_range(0., 0.5);
                        MaterialBuilder::Metal { albedo, fuzziness }
                    } else {
                        MaterialBuilder::Dielectric {
                            refractive_index: 1.5,
                        }
                    };
                    let little_ball = ShapeWithMaterialBuilder {
                        shape: ShapeBuilder::Sphere {
                            center,
                            radius: small_radius,
                        },
                        material,
                    };
                    world.push(little_ball);
                }
            }
        }

        world.push(ShapeWithMaterialBuilder {
            shape: ShapeBuilder::Sphere {
                center: Point::new(0., 1., 0.),
                radius: 1.0,
            },
            material: MaterialBuilder::Dielectric {
                refractive_index: 1.5,
            },
        });

        world.push(ShapeWithMaterialBuilder {
            shape: ShapeBuilder::Sphere {
                center: Point::new(-4., 1., 0.),
                radius: 1.0,
            },
            material: MaterialBuilder::Diffuse {
                albedo: Color::new(0.4, 0.2, 0.1),
            },
        });

        world.push(ShapeWithMaterialBuilder {
            shape: ShapeBuilder::Sphere {
                center: Point::new(4., 1., 0.),
                radius: 1.0,
            },
            material: MaterialBuilder::Metal {
                albedo: Color::new(0.7, 0.6, 0.5),
                fuzziness: 0.,
            },
        });

        Self {
            lookfrom: Point::new(13., 2., 3.),
            lookat: Point::new(0., 0., 0.),
            distance_to_focus: 10.,
            vertical_field_of_view: 20.,
            aperture: 0.1,
            aspect_ratio,
            view_up: Vec3::new(0., 1., 0.),
            world,
        }
    }

    pub fn build(&self) -> Scene {
        let camera = Camera::new(
            self.vertical_field_of_view,
            self.aspect_ratio,
            self.lookfrom,
            self.lookat,
            self.view_up,
            self.distance_to_focus,
            self.aperture,
        );

        let world = self.world.iter().map(|model| model.build()).collect();

        Scene::new(world, camera)
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
struct ShapeWithMaterialBuilder {
    shape: ShapeBuilder,
    material: MaterialBuilder,
}

impl ShapeWithMaterialBuilder {
    fn build(&self) -> Box<dyn Hittable + Send + Sync> {
        Box::new(ShapeWithMaterial::new(
            self.shape.build(),
            self.material.build(),
        ))
    }
}
