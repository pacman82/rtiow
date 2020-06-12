use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{dot, Point},
};

pub struct Sphere<M> {
    center: Point,
    radius: f64,
    material: M,
}

impl<M> Sphere<M> {
    pub fn new(center: Point, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M> Hittable for Sphere<M>
where
    M: Material,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

            HitRecord::from_outward_normal(t, ray, outward_normal, &self.material)
        })
    }
}
