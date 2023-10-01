use crate::vec3::Point3;


pub struct Perlin {
    random_floats: Vec<f64>,
    scale: f64,
    perm_x: Vec<u64>,
    perm_y: Vec<u64>,
    perm_z: Vec<u64>
}

impl Perlin {
    pub fn new(point_count: usize, scale: f64) -> Self {
        let mut rf = Vec::<f64>::with_capacity(point_count);
        for _ in 0..point_count {
            rf.push(rand::random::<f64>());
        }

        Perlin { 
            random_floats: rf,
            scale,
            perm_x: Perlin::generate_perlin_permutation(point_count),
            perm_y: Perlin::generate_perlin_permutation(point_count),
            perm_z: Perlin::generate_perlin_permutation(point_count)
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let i = ((self.scale * point.x()) as i64 & (self.random_floats.len() - 1) as i64) as usize;
        let j = ((self.scale * point.y()) as i64 & (self.random_floats.len() - 1) as i64) as usize;
        let k = ((self.scale * point.z()) as i64 & (self.random_floats.len() - 1) as i64) as usize;

        let index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        return self.random_floats[index]
    }

    fn generate_perlin_permutation(point_count: usize) -> Vec<u64> {
        let mut perm = Vec::<u64>::with_capacity(point_count);

        for i in 0..point_count {
            perm.push(i as u64);
        }

        Perlin::permute(&mut perm);
        perm
    }

    fn permute(array: &mut Vec<u64>) {
        for i in (array.len() - 1)..0 {
            let target = rand::random::<usize>() % i;
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
            scale: 4.0,
            perm_x: Perlin::generate_perlin_permutation(point_count),
            perm_y: Perlin::generate_perlin_permutation(point_count),
            perm_z: Perlin::generate_perlin_permutation(point_count)
        }
    }
}