use crate::{
    bvh::{into_bounding_volume_hierarchy, BoundedHittable},
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
    moving::Moving,
    scene::Scene,
    shape::Sphere,
    vec3::{Color, Point, Vec3}, texture::{Solid, Texture, Checkerd},
    perlin::Perlin,
};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, io, path::Path};

/// Serializable representation of a Scene. Used to persist scenes to '.toml' files.
#[derive(Serialize, Deserialize)]
pub struct SceneBuilder {
    pub camera: CameraBuilder,
    pub world: Vec<HittableBuilder>,
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
    pub vertical_field_of_view: f64,
    /// Width divided by height of the image to render.
    pub aspect_ratio: f64,
    /// Position of the camera.
    pub lookfrom: Point,
    /// Point the camera is looking at. Used to determine the direction of the camera.
    pub lookat: Point,
    pub view_up: Vec3,
    pub distance_to_focus: f64,
    pub aperture: f64,
    /// Use this for motion blur. Rays will be emitted randomly between t0=0 and t1=exposure_time.
    /// Can also be understood as the time it takes for the shutter to close.
    pub exposure_time: f64,
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

#[derive(Serialize, Deserialize, Clone)]
pub enum SurfaceBuilder {
    Diffuse { albedo: Color },
    Metal { albedo: Color, fuzziness: f64 },
    Dielectric { refractive_index: f64 },
    Checkered (Box<SurfaceBuilder>, Box<SurfaceBuilder>),
    Perlin { seed: u64, scale: f64 },
}

impl SurfaceBuilder {
    fn build(&self) -> Box<dyn Texture + Send + Sync> {
        match self {
            SurfaceBuilder::Diffuse { albedo } => Box::new(Solid(Lambertian::new(*albedo))),
            SurfaceBuilder::Metal { albedo, fuzziness } => Box::new(Solid(Metal::new(*albedo, *fuzziness))),
            SurfaceBuilder::Dielectric { refractive_index } => {
                Box::new(Solid(Dielectric::new(*refractive_index)))
            },
            SurfaceBuilder::Checkered(t0, t1) => {
                Box::new(Checkerd::new(t0.build(), t1.build()))
            }
            SurfaceBuilder::Perlin { seed, scale } => Box::new(Perlin::new( *seed, *scale )),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ShapeBuilder {
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
pub struct HittableBuilder {
    pub shape: ShapeBuilder,
    pub material: SurfaceBuilder,
    pub velocity: Option<Vec3>,
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
