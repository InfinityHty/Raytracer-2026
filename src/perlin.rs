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
        let i = (((point.x * 4.0) as i32) & 255) as usize;
        let j = (((point.y * 4.0) as i32) & 255) as usize;
        let k = (((point.z * 4.0) as i32) & 255) as usize;

        self.random_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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
