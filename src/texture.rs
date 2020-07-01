use crate::{material::Material, vec3::Point};

pub trait Texture {
    fn material(&self, u: f64, v: f64, point: &Point) -> &dyn Material;
}

impl<T> Texture for Box<T>
where
    T: Texture + ?Sized,
{
    fn material(&self, u: f64, v: f64, point: &Point) -> &dyn Material {
        self.as_ref().material(u, v, point)
    }
}

pub struct Solid<M>(pub M);

/// A solid texture made up entirely of one material.
impl<M> Texture for Solid<M>
where
    M: Material,
{
    fn material(&self, _u: f64, _v: f64, _point: &Point) -> &dyn Material {
        &self.0
    }
}

pub struct Checkerd<E, O> {
    even: E,
    odd: O,
}

impl<E, O> Checkerd<E, O> {
    pub fn new(t0: E, t1: O) -> Self {
        Self { even: t0, odd: t1 }
    }
}

impl<E, O> Texture for Checkerd<E, O>
where
    E: Texture,
    O: Texture,
{
    fn material(&self, u: f64, v: f64, point: &Point) -> &dyn Material {
        let frequency = 10.;
        let sines = (frequency * point.x()).sin()
            * (frequency * point.y()).sin()
            * (frequency * point.z()).sin();
        if sines < 0. {
            self.odd.material(u, v, point)
        } else {
            self.even.material(u, v, point)
        }
    }
}
