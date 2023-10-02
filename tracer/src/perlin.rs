use rand::Rng;

use crate::vec3::Point3;


pub struct Perlin {
    random_floats: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        let mut rf = Vec::<f64>::with_capacity(point_count);
        for _ in 0..point_count {
            rf.push(rand::random::<f64>());
        }

        Perlin { 
            random_floats: rf,
            perm_x: Perlin::generate_perlin_permutation(point_count),
            perm_y: Perlin::generate_perlin_permutation(point_count),
            perm_z: Perlin::generate_perlin_permutation(point_count)
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {        
        let mut u = point.x() - point.x().floor();
        let mut v = point.y() - point.y().floor();
        let mut w = point.z() - point.z().floor();

        u = u * u * (3.0 - (2.0 * u));
        v = v * v * (3.0 - (2.0 * v));
        w = w * w * (3.0 - (2.0 * w));

        let i = point.x().floor() as i64;
        let j = point.y().floor() as i64;
        let k = point.z().floor() as i64;
        let mut arr = vec![vec![vec![0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    arr[di][dj][dk] = self.random_floats[
                        self.perm_x[((i + di as i64) & 255 as i64) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255 as i64) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255 as i64) as usize]
                    ]
                }
            }
        }

        Perlin::trilinear_interpolation(&arr, u, v, w)
    }

    fn trilinear_interpolation(arr: &Vec<Vec<Vec<f64>>>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                for k in 0..arr[0][0].len() {
                    accum += arr[i][j][k]
                        * (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w));
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
        for i in (1..array.len()).rev() {
            let target = rand::thread_rng().gen_range(0..i);
            array.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let point_count = 256;
        let mut rf = Vec::<f64>::with_capacity(point_count);
        for _ in 0..point_count {
            rf.push(rand::random::<f64>());
        }

        Perlin { 
            random_floats: rf,
            perm_x: Perlin::generate_perlin_permutation(point_count),
            perm_y: Perlin::generate_perlin_permutation(point_count),
            perm_z: Perlin::generate_perlin_permutation(point_count)
        }
    }
}