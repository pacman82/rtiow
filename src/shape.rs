use crate::{
    bounding_box::{Aabb, BoundingBox},
    ray::Ray,
    vec3::{dot, Point, Vec3},
};

// A physical volume (without any material associated yet).
pub trait Shape {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub struct Intersection {
    pub t: f64,
    pub point: Point,
    pub front_face: bool,
    /// Always pointing against the intersecting ray.
    pub normal: Vec3,
}

impl Intersection {
    pub fn from_outward_normal(t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let point = ray.at(t);
        let front_face = dot(outward_normal, ray.direction) < 0.;
        Self {
            t,
            point,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        // Transform coordinats so sphere is in the center.
        let origin = ray.origin - self.center;
        // r^2 == (t * D + O) * (t * D + O)
        // r^2 == t^2 D*D + 2t*D*O + O*O
        // 0 == t^2 D*D + 2t*D*O + O*O - r^2
        let a = ray.direction.length_squared();
        let half_b = dot(ray.direction, origin);
        let c = origin.length_squared() - self.radius * self.radius;
        // a^2 x^2 + b x + c = 0 => x = (-b +- sqrt(b^2 - 4ac)) / 2a
        //                         4 * discriminant ^^^^^^^^^^
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            // No Solution to the equation => Sphere does not intersect with Ray.
            return None;
        }
        // Take the smaller of the potentiall two results as this is the intersection point there
        // the ray enters the sphere and not the one there it leaves it.
        let disc_sqrt = discriminant.sqrt();
        let t_1 = (-half_b - disc_sqrt) / a;
        let t = if t_min < t_1 && t_1 < t_max {
            Some(t_1)
        } else {
            // Try second solution, since first did not fit.
            let t_2 = (-half_b + disc_sqrt) / a;
            if t_min < t_2 && t_2 < t_max {
                Some(t_2)
            } else {
                // Solutions found, but none fit the t_min, t_max criteria
                None
            }
        };

        t.map(|t| {
            let point = ray.at(t);
            let outward_normal = (point - self.center) / self.radius;

            Intersection::from_outward_normal(t, ray, outward_normal)
        })
    }
}

impl BoundingBox for Sphere {
    fn bounding_box(&self, _exposure_time: f64) -> Aabb {
        let r3 = Vec3::new(self.radius, self.radius, self.radius);
        Aabb::new(self.center - r3, self.center + r3)
    }
}
