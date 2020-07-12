use crate::{
    material::{Lambertian, Material, ScatterResult},
    shape::Puncture,
    texture::Texture,
    vec3::{dot, Color, Point, Vec3},
};
use rand::{
    rngs::{StdRng, ThreadRng},
    seq::SliceRandom,
    SeedableRng,
};

pub struct Perlin {
    randoms: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
    scale: f64,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(seed: u64, scale: f64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut perm_x: Vec<_> = (0..Self::POINT_COUNT).collect();
        let mut perm_y: Vec<_> = (0..Self::POINT_COUNT).collect();
        let mut perm_z: Vec<_> = (0..Self::POINT_COUNT).collect();
        perm_x.shuffle(&mut rng);
        perm_y.shuffle(&mut rng);
        perm_z.shuffle(&mut rng);
        let randoms = (0..Self::POINT_COUNT)
            .map(|_| Vec3::random(&mut rng, -1., 1.))
            .collect();
        Self {
            randoms,
            perm_x,
            perm_y,
            perm_z,
            scale,
        }
    }

    fn noise(&self, p: &Point) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;

        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randoms[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]];
                }
            }
        }

        Self::perlin_interpolation(&c, u, v, w)
    }

    fn perlin_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // hermetian smoothing
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += if i == 0 { 1. - uu } else { uu }
                        * if j == 0 { 1. - vv } else { vv }
                        * if k == 0 { 1. - ww } else { ww }
                        * dot(c[i][j][k], weight);
                }
            }
        }
        accum
    }

    fn turbulence(&self, p: &Point, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        accum.abs()
    }
}

impl Texture for Perlin {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        punctured: &Puncture,
        incoming: &Vec3,
    ) -> Option<ScatterResult> {
        let albedo =
            // Color::new(1., 1., 1.) * 0.5 * (1.0 + self.noise(&(punctured.point * self.scale)));
            // Color::new(1., 1., 1.) * self.turbulence(&(punctured.point * self.scale), 7);
            Color::new(1.,1.,1.) * 0.5 * (1. + (self.scale * punctured.point.z() + 10. *self.turbulence(&punctured.point, 7)).sin());
        Lambertian::new(albedo).scatter(rng, incoming, &punctured.normal, punctured.front_face)
    }
}
