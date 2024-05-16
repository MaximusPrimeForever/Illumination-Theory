use rand::Rng;

use crate::math::vec3::{Point3, Vec3};


pub struct Perlin {
    random_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>
}

impl Perlin {
    #[allow(dead_code)]
    pub fn new(point_count: usize) -> Self {
        let mut rv = Vec::<Vec3>::with_capacity(point_count);
        for _ in 0..point_count {
            rv.push(Vec3::random_range(-1.0, 1.0));
        }

        Perlin { 
            random_vec: rv,
            perm_x: Perlin::generate_perlin_permutation(point_count),
            perm_y: Perlin::generate_perlin_permutation(point_count),
            perm_z: Perlin::generate_perlin_permutation(point_count)
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {        
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as i64;
        let j = point.y().floor() as i64;
        let k = point.z().floor() as i64;
        let mut arr = vec![vec![vec![Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    arr[di][dj][dk] = self.random_vec[
                          self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]
                    ]
                }
            }
        }

        Perlin::perlin_interpolation(&arr, u, v, w)
    }

    pub fn turbulence(&self, point: Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut temp_point = point;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        accum.abs()
    }

    fn perlin_interpolation(arr: &Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                for k in 0..arr[0][0].len() {
                    let weight_v = Vec3::new(
                        u - i as f64,
                        v - j as f64,
                        w - k as f64
                    );
                    accum +=
                          (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * arr[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }

    fn generate_perlin_permutation(point_count: usize) -> Vec<usize> {
        let mut perm = Vec::<usize>::with_capacity(point_count);

        for i in 0..point_count {
            perm.push(i as usize);
        }

        Perlin::permute(&mut perm);
        perm
    }

    fn permute(array: &mut Vec<usize>) {
        let mut rng = rand::thread_rng();

        for i in (1..array.len()).rev() {
            let target = rng.gen_range(0..i);
            array.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let point_count = 256;
        let mut rv = Vec::<Vec3>::with_capacity(point_count);
        for _ in 0..point_count {
            rv.push(Vec3::random_range(-1.0, 1.0));
        }

        Perlin { 
            random_vec: rv,
            perm_x: Perlin::generate_perlin_permutation(point_count),
            perm_y: Perlin::generate_perlin_permutation(point_count),
            perm_z: Perlin::generate_perlin_permutation(point_count)
        }
    }
}