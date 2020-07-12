use crate::{
    bounding_box::{Aabb, BoundingBox},
    hittable::{Hit, Hittable}, renderable::Renderable,
};
use std::cmp::Ordering;

pub trait BoundedHittable: Hittable + BoundingBox + Send + Sync {}
impl<T> BoundedHittable for T where T: Hittable + BoundingBox + Send + Sync {}

pub fn into_bounding_volume_hierarchy(
    mut hittables: Vec<Box<dyn BoundedHittable>>,
    exposure_time: f64,
) -> Box<dyn Renderable + Send + Sync> {
    match hittables.len() {
        0 => Box::new(hittables),
        1 => Box::new(hittables.drain(..).next().unwrap()),
        _ => {
            let (left, right) = split_list(hittables, exposure_time);
            let left = into_bvh_impl(left, exposure_time);
            let right = into_bvh_impl(right, exposure_time);
            // Note that the top level note is stored as a dyn Hittable trait object, not
            // BoundedHittable.
            Box::new(BvhNode::new(left, right, exposure_time))
        }
    }
}

fn into_bvh_impl(
    mut hittables: Vec<Box<dyn BoundedHittable>>,
    exposure_time: f64,
) -> Box<dyn BoundedHittable> {
    match hittables.len() {
        0 => panic!("Can't construct hierarchie from empty scene."),
        1 => hittables.drain(..).next().unwrap(),
        _ => {
            let (even, odd) = split_list(hittables, exposure_time);
            let left = into_bvh_impl(even, exposure_time);
            let right = into_bvh_impl(odd, exposure_time);
            Box::new(BvhNode::new(left, right, exposure_time))
        }
    }
}

fn split_list(
    mut hittables: Vec<Box<dyn BoundedHittable>>,
    exposure_time: f64,
) -> (Vec<Box<dyn BoundedHittable>>, Vec<Box<dyn BoundedHittable>>) {
    let npot = hittables.len().next_power_of_two();
    let axis = npot.trailing_zeros() % 3;

    hittables.sort_by(|l, r| sort_by_bounding_box(axis as usize, exposure_time, l, r));

    let mut count = 0;
    hittables.drain(..).partition(|_| {
        count += 1;
        count % 2 == 0
    })
}

fn sort_by_bounding_box(
    axis: usize,
    exposure_time: f64,
    a: &Box<dyn BoundedHittable>,
    b: &Box<dyn BoundedHittable>,
) -> Ordering {
    let a = a.bounding_box(exposure_time);
    let b = b.bounding_box(exposure_time);
    a.cmp_min_axis(axis, &b)
}

struct BvhNode {
    bounding_box: Aabb,
    left: Box<dyn BoundedHittable>,
    right: Box<dyn BoundedHittable>,
}

impl BvhNode {
    fn new(
        left: Box<dyn BoundedHittable>,
        right: Box<dyn BoundedHittable>,
        exposure_time: f64,
    ) -> Self {
        let bounding_box = Aabb::surrounding(
            &left.bounding_box(exposure_time),
            &right.bounding_box(exposure_time),
        );
        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        time: f64,
    ) -> Option<(f64, Hit)> {
        if self.bounding_box.hit(ray, t_min, t_max) {
            let l = self.left.hit(ray, t_min, t_max, time);
            let r = self.right.hit(ray, t_min, t_max, time);
            match (l, r) {
                (None, None) => None,
                (Some(hr), None) | (None, Some(hr)) => Some(hr),
                (Some(l), Some(r)) => {
                    if l.0 < r.0 {
                        Some(l)
                    } else {
                        Some(r)
                    }
                }
            }
        } else {
            None
        }
    }
}

impl BoundingBox for BvhNode {
    fn bounding_box(&self, _exposure_time: f64) -> Aabb {
        self.bounding_box
    }
}
