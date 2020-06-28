use crate::{
    bvh::{into_bounding_volume_hierarchy, BoundedHittable},
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    moving::Moving,
    scene::Scene,
    shape::Sphere,
    vec3::{Color, Point, Vec3},
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
    pub material: MaterialBuilder,
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
