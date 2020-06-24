use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};
use std::{cmp::Ordering, mem::swap};

pub trait BoundingBox {
    fn bounding_box(&self, exposure_time: f64) -> Aabb;
}

/// Axis aligned bounding box
#[derive(Clone, Copy)]
pub struct Aabb {
    /// All elements of `min` must be smaller or equal than those of `max`.
    min: Point,
    max: Point,
}

impl Aabb {
    pub fn new(min: Point, max: Point) -> Self {
        assert!(min
            .iter()
            .zip(max.iter())
            .all(|(emin, emax)| { emin <= emax }));
        Self { min, max }
    }

    pub fn surrounding(a: &Aabb, b: &Aabb) -> Self {
        Self {
            min: Point::new(
                a.min.x().min(b.min.x()),
                a.min.y().min(b.min.y()),
                a.min.z().min(b.min.z()),
            ),
            max: Point::new(
                a.max.x().max(b.max.x()),
                a.max.y().max(b.max.y()),
                a.max.z().max(b.max.z()),
            ),
        }
    }

    pub fn shifted(&self, direction: &Vec3) -> Self {
        Self {
            min: self.min + *direction,
            max: self.max + *direction,
        }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for d in 0..3 {
            let inv_direction = ray.direction[d].recip();
            let mut min = (self.min[d] - ray.origin[d]) * inv_direction;
            let mut max = (self.max[d] - ray.origin[d]) * inv_direction;
            if inv_direction.is_sign_negative() {
                swap(&mut min, &mut max);
            }
            t_min = t_min.max(min);
            t_max = t_max.min(max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn cmp_min_axis(&self, axis: usize, other: &Self) -> Ordering {
        self.min[axis].partial_cmp(&other.min[axis]).unwrap()
    }
}

impl<B> BoundingBox for Box<B>
where
    B: BoundingBox,
{
    fn bounding_box(&self, exposure_time: f64) -> Aabb {
        self.as_ref().bounding_box(exposure_time)
    }
}

impl<S, M> BoundingBox for (S, M)
where
    S: BoundingBox,
{
    fn bounding_box(&self, exposure_time: f64) -> Aabb {
        self.0.bounding_box(exposure_time)
    }
}
