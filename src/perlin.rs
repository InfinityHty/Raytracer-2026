use crate::vec3::Vec3;
use rand::{RngExt, rng};

pub struct PerlinNoise {
    random_float: [f64; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}
impl PerlinNoise {
    pub fn new() -> Self {
        let mut random_float: [f64; 256] = [0.0; 256];
        let mut perm_x: [usize; 256] = [0; 256];
        let mut perm_y: [usize; 256] = [0; 256];
        let mut perm_z: [usize; 256] = [0; 256];
        for it in &mut random_float {
            *it = PerlinNoise::random(0.0, 1.0);
        }
        PerlinNoise::generate(&mut perm_x);
        PerlinNoise::generate(&mut perm_y);
        PerlinNoise::generate(&mut perm_z);
        PerlinNoise {
            random_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, point: &Vec3) -> f64 {
        let mut u = point.x - point.x.floor();
        let mut v = point.y - point.y.floor();
        let mut w = point.z - point.z.floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];
        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_float[self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        self.trilinear_interp(c, u, v, w)
    }
    // 三维线性插值
    fn trilinear_interp(&self, c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accumulation = 0.0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accumulation += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accumulation
    }
    fn random(min: f64, max: f64) -> f64 {
        let mut rng = rng();
        rng.random_range(min..max)
    }
    fn generate(perm: &mut [usize; 256]) {
        #[allow(clippy::needless_range_loop)]
        for i in 0..256 {
            perm[i] = i;
        }
        let mut cnt = 255;
        while cnt != 0 {
            let target = PerlinNoise::random(0.0, cnt as f64) as usize;
            perm.swap(cnt, target);
            cnt -= 1;
        }
    }
}
