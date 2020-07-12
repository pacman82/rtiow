use crate::{
    material::{Material, ScatterResult},
    shape::Puncture,
    vec3::Vec3,
};
use rand::rngs::ThreadRng;

pub trait Texture {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        punctured: &Puncture,
        incoming: &Vec3,
    ) -> Option<ScatterResult>;
}

impl<T> Texture for Box<T>
where
    T: Texture + ?Sized,
{
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        punctured: &Puncture,
        incoming: &Vec3,
    ) -> Option<ScatterResult> {
        self.as_ref().scatter(rng, punctured, incoming)
    }
}

pub struct Solid<M>(pub M);

/// A solid texture made up entirely of one material.
impl<M> Texture for Solid<M>
where
    M: Material,
{
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        puncture: &Puncture,
        incoming: &Vec3,
    ) -> Option<ScatterResult> {
        self.0
            .scatter(rng, incoming, &puncture.normal, puncture.front_face)
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
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        puncture: &Puncture,
        incoming: &Vec3,
    ) -> Option<ScatterResult> {
        let point = &puncture.point;
        let frequency = 10.;
        let sines = (frequency * point.x()).sin()
            * (frequency * point.y()).sin()
            * (frequency * point.z()).sin();
        if sines < 0. {
            self.odd.scatter(rng, puncture, incoming)
        } else {
            self.even.scatter(rng, puncture, incoming)
        }
    }
}
